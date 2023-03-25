use std::io::{Write, Error, ErrorKind, Result, BufReader, BufRead};
use std::net::{TcpStream, ToSocketAddrs};
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

#[derive(Debug)]
struct Headers {
    response_status: String,
    content_type: String,
    transfer_encoding: bool,
    content_length: String
}

struct Response {
    headers: Headers,
    body: String
}

fn main() -> Result<()> {
    let req = Request {
        method: String::from("GET"),
        resource_path: String::from("/"),
        query_params: String::from("?param1=value"),
        host: String::from("www.google.com"),
        port: 80
    };
  
    // Connect to stream and send request
    let mut stream = req.make_stream_connection().unwrap();
    stream.write_all(req.prepare_http().as_bytes()).unwrap(); 
    
    // Init buf
    let mut buf_reader = BufReader::new(&stream);
    let mut temp_buf_storage = String::new();

    // Config response
    stream.set_read_timeout(Some(Duration::from_millis(300)));
    let mut headers = String::new();
    let mut response = String::new();

    // Get res status
    buf_reader.read_line(&mut temp_buf_storage)?;
    let response_status = String::from(&temp_buf_storage);

    // Read headers
    let mut content_type = String::new();
    let mut transfer_encoding = false;
    let mut content_length = String::new();
    loop {
        temp_buf_storage.clear();
        buf_reader.read_line(&mut temp_buf_storage)?;
   

        let line = temp_buf_storage.to_ascii_lowercase();
        
        if line.starts_with("transfer-encoding:") {
            transfer_encoding = true;
        };
        if line.starts_with("content-type:") {
            content_type.push_str(&temp_buf_storage);
        };
        if line.starts_with("content-length") {
            content_length.push_str(&temp_buf_storage);
        };
        
        if temp_buf_storage == "\r\n" {break};
        
        headers.push_str(&temp_buf_storage);

    }
    let head = Headers {
        response_status: response_status,
        content_type: content_type,
        transfer_encoding: transfer_encoding,
        content_length: content_length
    };
    println!("{:?}", head);
    Ok(())
}
