extern crate gotham;
extern crate hyper;
extern crate mime;

mod routes;

use hyper::server::Http;
use gotham::handler::NewHandlerService;

pub fn main() {
    let addr = "0.0.0.0:7878".parse().unwrap();

    let server = Http::new()
        .bind(&addr, NewHandlerService::new(routes::router()))
        .unwrap();

    println!("Listening on http://{}", server.local_addr().unwrap());

    server.run().unwrap();
}
