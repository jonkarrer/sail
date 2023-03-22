use std::io::{Read,Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("www.google.com:80").unwrap();
    let request = b"GET / HTTP/1.1\r\nHost: www.google.com\r\n\r\n";
    stream.write_all(request); 
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
