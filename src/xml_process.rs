use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;

use minidom::Element;
use xjbutil::minhttpd::{HttpBody, HttpHeaders, HttpParams, HttpResponse, HttpUri};

use crate::http_to_py::http_to_py;
use crate::instantiate::instantiate;

const XML_NS: &'static str = "active-xhtml";

pub fn process_xml_file(
    file_name: &str,
    file_content: String,
    uri: HttpUri,
    headers: HttpHeaders,
    params: HttpParams,
    body: HttpBody
) -> Result<HttpResponse, Box<dyn Error>> {
    let xml_dom = file_content.parse::<Element>()?;
    if !xml_dom.has_child("html", XML_NS) {
        return Err(format!("in file {}: XML should at least contain document node", file_name).into());
    }

    let script_node = xml_dom.get_child("script", XML_NS);
    return if let Some(script) = script_node {
        let script = load_script_string(script)?;
        let mut py_code_buf = http_to_py(&uri, &headers, &params, &body)?;
        write!(py_code_buf, "\n\n{}", script)?;

        let mut child = Command::new("python")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let mut stdin = child.stdin.take().unwrap();
        thread::spawn(move || {
            let _ = stdin.write_all(&py_code_buf);
        });

        let output = &child.wait_with_output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let vars = py_resp_to_vars(&stdout)?;

        if let Some(fatal_reason) = vars.get("$FATAL") {
            return Err(format!("script {} fatal error: {}", script, fatal_reason).into());
        }

        if let Some(redirection) = vars.get("$REDIRECT") {
            return Ok(HttpResponse::builder()
                .set_code(301)
                .add_header("Location", redirection)
                .build());
        }

        let document_node = xml_dom.get_child("html", XML_NS).unwrap();
        let instantiated_node = instantiate(document_node, vars);

        let mut buf = Vec::new();
        instantiated_node.write_to(&mut buf)?;

        let document = String::from_utf8_lossy(&buf).to_string();
        Ok(HttpResponse::builder()
            .add_header("Content-Type", "text/html")
            .set_payload(document)
            .build())
    } else {
        let document_node = xml_dom.get_child("html", XML_NS).unwrap();
        let mut buf = Vec::new();
        document_node.write_to(&mut buf)?;

        let document = String::from_utf8_lossy(&buf).to_string();
        Ok(HttpResponse::builder()
            .add_header("Content-Type", "text/html")
            .set_payload(document)
            .build())
    }
}

fn load_script_string(script_node: &Element) -> Result<String, Box<dyn Error>> {
    if let Some(file_name) = script_node.attr("ref") {
        Ok(read_to_string(file_name)?)
    } else {
        Ok(script_node.text())
    }
}

fn py_resp_to_vars(resp: &String) -> Result<HashMap<&str, String>, Box<dyn Error>> {
    let lines = resp.trim().split('\n');
    let mut output_variables = HashMap::new();
    for line in lines {
        let trimmed_line = line.trim();
        if !(trimmed_line.starts_with('$') && trimmed_line.contains('=')) {
            continue;
        }

        let parts = trimmed_line.split('=').collect::<Vec<_>>();
        let var_name = parts[0];
        let value = urlencoding::decode(parts[1])?.to_string();

        output_variables.insert(var_name, value);
    }

    Ok(output_variables)
}
