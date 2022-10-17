use std::error::Error;
use std::io::Write;

use xjbutil::minhttpd::{HttpBody, HttpHeaders, HttpParams, HttpUri};

pub fn http_to_py(
    uri: &HttpUri,
    headers: &HttpHeaders,
    params: &HttpParams,
    body: &HttpBody,
    listen_socket_addr: &str,
    session_auth_token: &str
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    write!(buf, "from x import *\n")?;

    write!(buf, "xm_set_http_uri(\"{}\")\n", uri)?;
    write!(buf, "xm_set_socket_addr(\"{}\")\n", listen_socket_addr)?;
    write!(buf, "xm_set_auth_token(\"{}\")\n", session_auth_token)?;

    for (header_name, header_value) in headers.iter() {
        if let Ok(header_value) = urlencoding::decode(header_value) {
            write!(buf, "xm_add_http_header(\"{}\", \"{}\")\n", header_name, header_value)?;
        }
    }

    for (param_name, param_value) in params.iter() {
        write!(buf, "xm_add_http_query(\"{}\", \"{}\")\n", param_name, param_value)?;
    }

    if let Some(body) = body {
        write!(buf, "xm_set_http_body(\"{}\")\n", body)?;
        if let Some("application/x-www-form-urlencoded") =
            headers.get("content-type").map(String::as_str)
        {
            write!(buf, "xm_add_parsed_http_body()\n")?;

            for kv_pair in body.split('&') {
                let kv = kv_pair.split('=').collect::<Vec<_>>();
                if kv.len() != 2 {
                    continue;
                }
                write!(buf, "xm_add_parsed_http_body_item(\"{}\", \"{}\")\n", kv[0], kv[1])?;
            }
        }
    }

    Ok(buf)
}
