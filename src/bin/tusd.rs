extern crate hyper;
extern crate clap;

extern crate tus;

use hyper::server::Server;
use clap::{Arg, App};

fn main() {
    let matches = App::new("tusd")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Rio Kierkels <riokierkels@gmail.com>")
                      .arg(Arg::with_name("host")
                               .short("h")
                               .long("host")
                               .help("address to bind to")
                               .takes_value(true))
                      .arg(Arg::with_name("port")
                               .short("p")
                               .long("port")
                               .help("port to listen on")
                               .takes_value(true))
                      .get_matches();

    let host = matches.value_of("host").unwrap_or("localhost");
    let port = matches.value_of("port").unwrap_or("8888");

    let bind_address = &*format!("{}:{}", host, port);

    println!("Listening on {}", bind_address);

    Server::http(bind_address).unwrap().handle(tus::tus_handler).unwrap();
}
