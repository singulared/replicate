extern crate hyper;
extern crate futures;

use futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::Error;

struct ReplicateService;


impl Service for ReplicateService {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = Error;

    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        // println!("Request debug {:?}", _request);
        
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        let body = format!("Method: {method}\nURI: {uri}", method=request.method(), uri=request.uri());

        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(body.len() as u64))
                .with_body(body)
        ))
    }
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(ReplicateService)).unwrap();
    server.run().unwrap();
}
