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

    while res_bytes[res_bytes.len() - 1] != 125 {
        res_bytes.remove(res_bytes.len() - 1);
    }

    let divisions = (res_bytes.len() / 80) - 1;
    let mut i = 80;

    while i < divisions * 80 {
        res_bytes.insert(i, 10);
        i+=80;
    } 

    let new_string = String::from_utf8(res_bytes).unwrap(); 
    println!("{:?}", res.header);
    println!("{}", new_string);
}
