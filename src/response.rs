extern crate native_tls;

use native_tls::TlsStream;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;


pub fn parse_response(mut stream: TlsStream<TcpStream>) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buf = String::new();
    
    loop {
        buf.clear();
        match buf_reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                if buf == "\r\n" {
                    buf.clear();
                    break;
                };
                println!("{}",&buf.trim_end_matches("\r\n"));
            }
            Err(_) => println!("Failed to read header line"),
        }
    }
}
