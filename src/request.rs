
pub struct Request {
    pub method: String,
    pub resource_path: String,
    pub query_params: String,
    pub host: String,
    pub port: i16,
}

impl Request {
  pub fn http(&self) -> String {
      return format!(
          "{} {}{} HTTP/1.1\r\nHost: {}\r\n\r\n",
          &self.method, &self.resource_path, &self.query_params, &self.host
      );
  }
}