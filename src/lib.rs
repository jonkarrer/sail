mod connect;
pub mod request;
pub mod response;

pub use request::Request;
pub use response::Response;
use std::io::Write;

pub fn send(req: Request) -> Response {
    let mut stream = connect::make_stream_connection(&req.host, &req.port).unwrap();
    stream.write_all(req.http().as_bytes()).unwrap();

    response::parse_response(stream)
}
