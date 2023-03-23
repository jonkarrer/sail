use std::io::{Read, Write, Error, ErrorKind};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;


struct Request {
    method: String,
    resource_path: String,
    host: String,
    port: i16
}

impl Request {
    fn build(&self) -> String {
        let r = format!("{} {} HTTP/1.1\r\nHost: {}\r\n\r\n", &self.method, &self.resource_path, &self.host);
        return r;
    }
    fn make_stream_connection(&self) -> Result<TcpStream, Error> {
        let address = format!("{}:{}", &self.host, &self.port);
        let possible_ip_addresses = address.to_socket_addrs().unwrap();
        
        for addr in possible_ip_addresses {
            let ok_connection = match TcpStream::connect_timeout(&addr, Duration::new(2, 0)) {
                Ok(stream) => return Ok(stream),
                Err(_e) => {
                    continue
                }
            };
        }
        Err(Error::new(ErrorKind::Other, "Failed to connect to any address"))
    }
}

fn main() {
    let req = Request {
        method: String::from("GET"),
        resource_path: String::from("/"),
        host: String::from("www.google.com"),
        port: 80
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
