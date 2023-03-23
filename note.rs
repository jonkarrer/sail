
use std::io::{self, Read};
use std::net::TcpStream;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("example.com:80")?;

    // Set a read timeout of 5 seconds
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                // do something with the data
                println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Handle the timeout error
                eprintln!("Read timed out");
                break;
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }

    Ok(())
}
