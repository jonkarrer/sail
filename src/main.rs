use std::io::Write;

mod request;
mod response;
use request::Request;

fn main() {
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

    let mut res_bytes = res.body.into_bytes();

    while res_bytes[0] != 123 {
       res_bytes.remove(0); 
    }

    
    println!("{:?}", res.header);
    println!("{:?}", res_bytes);
}
