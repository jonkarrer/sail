use std::io::{Read,Write};
use std::net::TcpStream;

struct Request {
    method: String,
    resource_path: String,
    host: String
}

impl Request {
    fn build(&self) -> String {
        let r = format!("{} {} HTTP/1.1\r\nHost: {}\r\n\r\n", &self.method, &self.resource_path, &self.host);
        return r;
    }
}
fn main() {
    let req = Request {
        method: String::from("GET"),
        resource_path: String::from("/api"),
        host: String::from("localhost")
    };
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.write_all(req.build().as_bytes()); 
    let mut buffer = [0;1024];

    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();

        println!("Reading");
        if bytes_read == 0 {
            break
        }

        let data = &buffer[..bytes_read];
        println!("Res {}",String::from_utf8_lossy(data));
    }
}
