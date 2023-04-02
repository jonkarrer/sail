extern crate native_tls;

use self::native_tls::{TlsConnector, TlsStream};
use std::io::{Error, ErrorKind, Result};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn make_stream_connection(host: &str, port: &i16) -> Result<TlsStream<TcpStream>> {
    let address = format!("{}:{}", host, port);
    let possible_ip_addresses = address.to_socket_addrs().expect("Socket Address Not Found");

    let connector = TlsConnector::new().expect("Tls Connector Failed");
    for addr in possible_ip_addresses {
        match TcpStream::connect_timeout(&addr, Duration::new(2, 0)) {
            Ok(stream) => {
                stream
                    .set_read_timeout(Some(Duration::from_millis(300)))
                    .expect("Stream Timeout Failed");
                let secure_stream = connector
                    .connect(&host, stream)
                    .expect("Secure Stream Failed");
                return Ok(secure_stream);
            }
            Err(_e) => continue,
        };
    }
    Err(Error::new(
        ErrorKind::Other,
        "Failed to connect to any address",
    ))
}
