use std::io::{Error, ErrorKind, Result};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
pub struct Request {
   pub method: String,
   pub resource_path: String,
   pub query_params: String,
   pub host: String,
   pub port: i16
}

impl Request {
    pub fn prepare_http(&self) -> String {
        return format!("{} {}{} HTTP/1.1\r\nHost: {}\r\n\r\n", &self.method, &self.resource_path, &self.query_params, &self.host);
    }

    pub fn make_stream_connection(&self) -> Result<TcpStream> {
        let address = format!("{}:{}", &self.host, &self.port);
        let possible_ip_addresses = address.to_socket_addrs().unwrap();
        
        for addr in possible_ip_addresses {
            match TcpStream::connect_timeout(&addr, Duration::new(2, 0)) {
                Ok(stream) => return Ok(stream),
                Err(_e) => {
                    continue
                }
            };
        }
        Err(Error::new(ErrorKind::Other, "Failed to connect to any address"))
    }
}
