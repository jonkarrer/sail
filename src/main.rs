use sail::*;

fn main() {
    let request = Request {
        method: "GET".to_string(),
        resource_path: "/".to_string(),
        query_params: "".to_string(),
        host: "example.com".to_string(),
        port: 443,
    };
    println!("{}", request.http());
    dbg!(send(request).body);
}
