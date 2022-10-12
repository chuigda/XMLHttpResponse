use std::error::Error;
use std::io::Write;

use xjbutil::minhttpd::{HttpBody, HttpHeaders, HttpParams, HttpUri};

const XPY_CONTENT: &'static str = include_str!("x.py");

pub fn http_to_py(
    uri: &HttpUri,
    headers: &HttpHeaders,
    params: &HttpParams,
    body: &HttpBody
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    write!(buf, "HttpUri = \"{}\"\n", uri)?;
    write!(buf, "HttpHeaders = {{}}\n")?;
    for (header_name, header_value) in headers.iter() {
        write!(buf, "HttpHeaders[\"{}\"] = \"{}\"\n", header_name, header_value)?;
    }
    write!(buf, "HttpQuery = {{}}\n")?;
    for (param_name, param_value) in params.iter() {
        write!(buf, "HttpQuery[\"{}\"] = \"{}\"\n", param_name, param_value)?;
    }
    if let Some(body) = body {
        write!(buf, "HttpBody = \"{}\"\n", body)?;
        if let Some("application/x-www-form-urlencoded") =
            headers.get("content-type").map(String::as_str)
        {
            write!(buf, "ParsedHttpBody = {{}}\n")?;
            for kv_pair in body.split('&') {
                let kv = kv_pair.split('=').collect::<Vec<_>>();
                if kv.len() != 2 {
                    continue;
                }
                write!(buf, "ParsedHttpBody[\"{}\"] = \"{}\"\n", kv[0], kv[1])?;
            }
        } else {
            write!(buf, "ParsedHttpBody = None\n")?;
        }
    } else {
        write!(buf, "HttpBody = None\n")?;
    }
    write!(buf, "{}", XPY_CONTENT)?;

    Ok(buf)
}
