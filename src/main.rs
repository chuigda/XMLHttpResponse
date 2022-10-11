mod xml_process;
mod http_to_py;
mod instantiator;

use std::net::SocketAddrV4;
use std::str::FromStr;

use xjbutil::liberty::Liberty;
use xjbutil::minhttpd::MinHttpd;

use crate::xml_process::process_xml_file;

fn main() {
    Liberty::liberty(false, true);

    let mut httpd = MinHttpd::with_logger(|_, s| eprintln!("{}", s));

    httpd.route_fn(
        "",
        |uri, headers, params, body| process_xml_file("www/example.xml", uri, headers, params, body)
    );

    httpd.serve(SocketAddrV4::from_str("127.0.0.1:3080").unwrap()).unwrap();
}
