use std::io::{BufReader, BufRead};
use std::net::{TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct Header {
    response_status: String,
    content_type: String,
    transfer_encoding: bool,
    content_length: String,
    raw_data: String
}

pub fn parse_header(stream: TcpStream) -> Header {
    let mut buf_reader = BufReader::new(&stream);
    let mut buf = String::new();
    
    buf_reader.read_line(&mut buf).unwrap();
    let mut response_status = String::new();
    response_status.push_str(&buf.trim_end_matches("\r\n"));

    let mut headers = Header {
        response_status: response_status,
        content_type: String::new(),
        transfer_encoding: false,
        content_length: String::new(),
        raw_data: String::new()
    };

    // Read lines in header
    loop {
        buf.clear();
        buf_reader.read_line(&mut buf).unwrap();
   
        let line = buf.to_ascii_lowercase();
        
        if line.starts_with("transfer-encoding:") {
            headers.transfer_encoding = true;
        };

        if line.starts_with("content-type:") {
            headers.content_type.push_str(&buf.trim_end_matches("\r\n"));
        };

        if line.starts_with("content-length") {
            headers.content_length.push_str(&buf.trim_end_matches("\r\n"));
        };
        
        if buf == "\r\n" {break};
        
        headers.raw_data.push_str(&buf.trim());
    }
    return headers;
}

