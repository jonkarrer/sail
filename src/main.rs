use std::io::{Read, Write, Error, ErrorKind, Result};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;

struct Request {
    method: String,
    resource_path: String,
    query_params: String,
    host: String,
    port: i16
}

impl Request {
    fn prepare_http(&self) -> String {
        return format!("{} {}{} HTTP/1.1\r\nHost: {}\r\n\r\n", &self.method, &self.resource_path, &self.query_params, &self.host);
    }

    fn make_stream_connection(&self) -> Result<TcpStream> {
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

fn main() -> Result<()> {
    let req = Request {
        method: String::from("GET"),
        resource_path: String::from("/"),
        query_params: String::from("?param1=value"),
        host: String::from("www.google.com"),
        port: 80
    };
   
    let mut stream = req.make_stream_connection().unwrap();
    stream.write_all(req.prepare_http().as_bytes()).unwrap(); 
    let mut buffer = [0;2096];
    let mut response = String::new();

    stream.set_read_timeout(Some(Duration::from_millis(300)));
    
    let mut i = 0;
    loop {
        i+=1;
        println!("KiloBytes {}", i);
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => response.push_str(&String::from_utf8_lossy(&buffer[..bytes_read])),
            Err(_) => break
        }
    }
    println!("{}", response);
    Ok(())
}
