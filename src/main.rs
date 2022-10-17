mod eval;
mod cfg;
mod xml_process;
mod http_to_py;
mod instantiate;
mod scan_dir;

use std::collections::{HashMap, HashSet};
use std::env::set_current_dir;
use std::fs::read_to_string;
use std::net::SocketAddrV4;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use xjbutil::liberty::Liberty;
use xjbutil::minhttpd::{HttpResponse, MinHttpd};
use xjbutil::rand::random_string;
use xjbutil::std_ext::ExpectSilentExt;

use crate::cfg::XmlHttpConfig;
use crate::scan_dir::find_resp_file_by_uri;
use crate::xml_process::process_xml_file;

fn main() {
    Liberty::liberty(false, true);

    let config = read_to_string("config.toml").expect_silent("cannot intake 'config.toml'");
    let config = toml::from_str::<XmlHttpConfig>(&config).expect_silent("cannot parse 'config.toml'");

    set_current_dir(config.work_dir).expect_silent("failed changing to working directory");

    let mut httpd = MinHttpd::with_logger(|_, s| eprintln!("{}", s));
    let mut allowed_file_extension = HashSet::new();
    allowed_file_extension.insert("xml".to_string());
    allowed_file_extension.insert("html".to_string());
    for mime in config.mimes.iter() {
        allowed_file_extension.insert((&mime.ext[1..]).to_string());
    }

    let session_storage = Arc::new(Mutex::new(HashMap::new()));
    let session_auth_token = random_string(32).to_ascii_lowercase();

    let storage_clone = session_storage.clone();
    let auth_token_clone = session_auth_token.clone();
    httpd.route(
        "/xhr-xapi/session/set",
        Box::new(
            move |_, headers, params, body| {
                let auth_token = headers.get("x-xhr-api-auth-token").ok_or("where is your token?")?;
                if auth_token != auth_token_clone.as_str() {
                    return Err("you mistaken your access token".into());
                }

                let session_token = params.get("session").ok_or("where is your session id?")?;
                let body = body.ok_or("where is your session content")?;
                dbg!(&body);
                storage_clone.lock().unwrap().insert(session_token.clone(), body);

                Ok(HttpResponse::builder().set_payload("ok").build())
            }
        )
    );

    let storage_clone = session_storage.clone();
    let auth_token_clone = session_auth_token.clone();
    httpd.route(
        "/xhr-xapi/session/get",
        Box::new(
            move |_, headers, params, _| {
                let auth_token = headers.get("x-xhr-api-auth-token").ok_or("where is your token?")?;
                if auth_token != auth_token_clone.as_str() {
                    return Err("you mistaken your access token".into());
                }

                let session_token = params.get("session").ok_or("where is your session id?")?;
                if let Some(session_content) = storage_clone.lock().unwrap().get(session_token) {
                    Ok(HttpResponse::builder()
                        .set_payload(session_content)
                        .build())
                } else {
                    Ok(HttpResponse::builder()
                        .set_code(404)
                        .set_payload("no session content")
                        .build())
                }
            }
        )
    );

    let auth_token_clone = session_auth_token.clone();
    let socket_addr = config.socket_addr.clone();
    httpd.route(
        "",
        Box::new(
            move |uri, headers, params, body| {
                let (file_name, content) = find_resp_file_by_uri(&uri, &allowed_file_extension)?;
                if file_name.ends_with(".html") {
                    return Ok(HttpResponse::builder()
                        .add_header("Content-Type", "text/html")
                        .set_payload(content)
                        .build());
                }

                for mime in config.mimes.iter() {
                    if file_name.ends_with(&mime.ext) {
                        return Ok(HttpResponse::builder()
                            .add_header("Content-Type", &mime.mime)
                            .set_payload(content)
                            .build())
                    }
                }

                process_xml_file(
                    &file_name,
                    content,
                    uri,
                    headers,
                    params,
                    body,
                    socket_addr.as_str(),
                    auth_token_clone.as_str()
                )
            }
        )
    );

    httpd.serve(SocketAddrV4::from_str(&config.socket_addr).unwrap()).unwrap();
}
