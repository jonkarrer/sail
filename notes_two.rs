
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("example.com:80")?;
    stream.write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")?;

    let mut reader = BufReader::new(&stream);
    let mut buf = String::new();

    // Read response status line
    reader.read_line(&mut buf)?;

    // Read response headers
    loop {
        buf.clear();
        reader.read_line(&mut buf)?;
        if buf == "\r\n" {
            break;
        }
        println!("{}", buf.trim());
    }

    // Read response body
    loop {
        buf.clear();
        let size = {
            let mut size_str = String::new();
            reader.read_line(&mut size_str)?;
            isize::from_str_radix(size_str.trim(), 16).unwrap()
        };
        if size == 0 {
            break;
        }
        reader.by_ref().take(size as u64).read_to_string(&mut buf)?;
        println!("{}", buf);
    }

    Ok(())
}
