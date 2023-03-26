use std::io::{Result, Write};

mod request;
mod response;
use request::Request;

fn main() -> Result<()> {
    //TODO Get request from CLI
    // TODO Make a parser for input of http string
    let req = Request {
        method: String::from("GET"),
        resource_path: String::from("/products"),
        query_params: String::new(),
        host: String::from("www.dummyjson.com"),
        port: 443,
    };

    // Connect to stream and send request
    let mut stream = req.make_stream_connection().unwrap();
    stream.write_all(req.prepare_http().as_bytes()).unwrap();

    let res = response::parse_response(stream);
    println!("{:?}", res);
    Ok(())
}
