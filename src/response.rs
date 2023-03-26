extern crate native_tls;

use native_tls::TlsStream;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Header {
    response_status: String,
    content_type: String,
    raw: String,
}

#[derive(Debug)]
pub struct Response {
    header: Header,
    body: String,
}

pub fn parse_response(mut stream: TlsStream<TcpStream>) -> Response {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buf = String::new();

    let mut header = Header {
        response_status: String::new(),
        content_type: String::new(),
        raw: String::new(),
    };

    // Response Status Code
    buf_reader.read_line(&mut buf).unwrap();
    header
        .response_status
        .push_str(&buf.trim_end_matches("\r\n"));

    // Read lines in header
    loop {
        buf.clear();
        match buf_reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                if buf == "\r\n" {
                    buf.clear();
                    break;
                };
                let line = buf.to_ascii_lowercase();
                if line.starts_with("content-type:") {
                    header.content_type.push_str(&buf.trim_end_matches("\r\n"));
                };
                header.raw.push_str(&buf.trim_end_matches("\r\n"));
            }
            Err(_) => println!("Failed to read header line"),
        }
    }

    let mut response = Response {
        header: header,
        body: String::new(),
    };

    loop {
        buf.clear();
        match buf_reader.read_line(&mut buf) {
            Ok(0) => {
                response.body.push_str(&buf.trim());
                break;
            }
            Ok(_) => {
                if &buf == "" {
                    break;
                };
                response.body.push_str(&buf.trim());
            }
            Err(_) => break,
        };
        if &buf == "" {
            break;
        };
    }

    return response;
}
