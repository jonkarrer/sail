use std::io::Write;
use std::env;
mod request;
mod response;
use request::Request;

fn main() {

    let cli_args: Vec<String> = env::args().collect();

    let request_address = &cli_args[1];

    println!("{}", request_address);
    
    // Parse
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
}
