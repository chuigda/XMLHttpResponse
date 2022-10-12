mod xml_process;
mod http_to_py;
mod instantiate;
mod scan_dir;
mod cfg;

use std::collections::HashSet;
use std::env::set_current_dir;
use std::fs::read_to_string;
use std::net::SocketAddrV4;
use std::str::FromStr;

use xjbutil::liberty::Liberty;
use xjbutil::minhttpd::{HttpResponse, MinHttpd};
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

                process_xml_file(&file_name, content, uri, headers, params, body)
            }
        )
    );

    httpd.serve(SocketAddrV4::from_str(&config.socket_addr).unwrap()).unwrap();
}
