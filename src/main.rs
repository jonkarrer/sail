use std::io::{Write, Result};
use std::time::Duration;

mod header;
mod request;
use request::Request;

fn main() -> Result<()> {
    let req = Request {
        method: String::from("GET"),
        resource_path: String::from("/"),
        query_params: String::from("?param1=value"),
        host: String::from("www.google.com"),
        port: 80
    };
  
    // Connect to stream and send request
    let mut stream = req.make_stream_connection().unwrap();
    stream.set_read_timeout(Some(Duration::from_millis(300))).unwrap();
    stream.write_all(req.prepare_http().as_bytes()).unwrap(); 
    

    let headers = header::parse_header(stream); 
    println!("{:?}", headers);
    Ok(())
}
