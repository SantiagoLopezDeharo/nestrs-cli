use std::collections::HashMap;

pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    fn status_text(code: u16) -> &'static str {
        match code {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "Unknown",
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let status_line = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            Self::status_text(self.status_code)
        );
        let mut response = status_line;

        let has_content_length = self.headers.contains_key("Content-Length");
        let has_connection = self.headers.contains_key("Connection");

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        if !has_content_length {
            response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        }

        if !has_connection {
            response.push_str("Connection: close\r\n");
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response.into_bytes()
    }
}
