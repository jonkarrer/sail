use std::io::{Read,Write};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};

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
    fn make_stream_connection(&self) -> Result<TcpStream, std::io::Error> {
        let host = "127.0.0.1";
        let port = 8080;
        let address = format!("{}:{}", host, port);
        return TcpStream::connect(address);
    }
}

fn main() {
    let req = Request {
        method: String::from("GET"),
        resource_path: String::from("/api"),
        host: String::from("localhost")
    };
   
    let mut stream = req.make_stream_connection().unwrap();
    stream.write_all(req.build().as_bytes()).unwrap(); 
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
