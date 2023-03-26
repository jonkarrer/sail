use std::io::{BufReader, BufRead};
use std::net::{TcpStream};

#[derive(Debug)]
pub struct Header {
    response_status: String,
    content_type: String,
}

pub fn parse_header(stream: TcpStream) -> Header {
    let mut buf_reader = BufReader::new(&stream);
    let mut buf = String::new();
   
    let mut header = Header {
        response_status: String::new(),
        content_type: String::new(),
    };
    
    // Response Status Code
    buf_reader.read_line(&mut buf).unwrap();
    header.response_status.push_str(&buf.trim_end_matches("\r\n"));

    // Read lines in header
    loop {
        buf.clear();
        buf_reader.read_line(&mut buf).unwrap();
   
        let line = buf.to_ascii_lowercase();

        if line.starts_with("content-type:") {
            header.content_type.push_str(&buf.trim_end_matches("\r\n"));
            break
        };

        if buf == "\r\n" {break};
    }
    return header;
}

