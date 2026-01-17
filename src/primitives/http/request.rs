use std::collections::HashMap;
use std::fmt;
use tokio::net::TcpStream;

pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub stream: TcpStream,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Method: {}", self.method)?;
        writeln!(f, "URL: {}", self.url)?;
        writeln!(f, "Headers: {:#?}", self.headers)?;
        writeln!(f, "Body: {}", self.body)
    }
}