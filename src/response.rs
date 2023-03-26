use std::io::{BufReader, BufRead, Read};
use std::net::{TcpStream};

#[derive(Debug)]
pub struct Header {
    response_status: String,
    content_type: String,
}

#[derive(Debug)]
pub struct Response {
    header: Header,
    body: String
}

pub fn parse_response(stream: TcpStream) -> Response {
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
        };
        if buf == "\r\n" {
            buf.clear();
            break
        };
    }
  
    let mut response = Response {
        header: header,
        body: String::new()
    };
   

    loop {
        buf.clear();
        match buf_reader.read_line(&mut buf) {
            Ok(0) => {
                response.body.push_str(&buf.trim());
                break
            },
            Ok(_) => {
                if &buf == "" {
                    break
                };
                response.body.push_str(&buf.trim());
            },
            Err(_) => println!("Failed to read body line")
        };
        if &buf == "" {
            break
        };
    }
  
    return response;
    

}

