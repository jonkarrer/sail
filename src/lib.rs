pub mod request;
pub mod response;
mod connect;

use std::io::Write;
use response::Response;
use request::Request;

pub fn send(req: Request) -> Response {
    // Connect to stream and send request
    let mut stream = connect::make_stream_connection(&req.host, &req.port).unwrap();
    stream.write_all(req.http().as_bytes()).unwrap();

    return response::parse_response(stream);
}
