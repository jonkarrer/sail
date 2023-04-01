mod connect;
mod request;
mod response;

use request::Request;
use std::io::Write;

fn main() {
    let req = Request {
        method: String::from("GET"),
        resource_path: String::from("/products"),
        query_params: String::new(),
        host: String::from("www.dummyjson.com"),
        port: 443,
    };

    // Connect to stream and send request
    let mut stream = connect::make_stream_connection(&req.host, &req.port).unwrap();
    stream.write_all(req.http().as_bytes()).unwrap();

    let res = response::parse_response(stream);

    println!("{}", res.body);
}
