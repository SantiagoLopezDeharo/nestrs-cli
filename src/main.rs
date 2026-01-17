use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

mod domain;
mod primitives;
mod routing;
use primitives::http::request::Request;
use routing::{init, init_routes, route};

async fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_request = Vec::new();
    let mut line = String::new();

    while buf_reader.read_line(&mut line).await.unwrap() > 0 {
        let trimmed = line.trim_end().to_string();
        if trimmed.is_empty() {
            break;
        }
        http_request.push(trimmed);
        line.clear();
    }

    let (method, url, _version) = if let Some(request_line) = http_request.get(0) {
        let mut parts = request_line.split_whitespace();
        (
            parts.next().unwrap_or("").to_string(),
            parts.next().unwrap_or("").to_string(),
            parts.next().unwrap_or("").to_string(),
        )
    } else {
        ("".to_string(), "".to_string(), "".to_string())
    };

    let mut headers = HashMap::new();
    for line in http_request.iter().skip(1) {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    let mut body = String::new();
    if let Some(content_length) = headers.get("Content-Length") {
        if let Ok(len) = content_length.parse::<usize>() {
            let mut buf = vec![0u8; len];
            buf_reader.read_exact(&mut buf).await.unwrap();
            body = String::from_utf8_lossy(&buf).to_string();
        }
    }

    let mut request = Request {
        method,
        url,
        headers,
        body,
        stream,
    };

    let response = route(&request).await;

    println!("{}", request);
    request
        .stream
        .write_all(&response.to_bytes())
        .await
        .unwrap();
    let _ = request.stream.shutdown().await;
}

fn main() {
    println!("Hello, world!");
    init(init_routes());
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let local = tokio::task::LocalSet::new();

    runtime.block_on(local.run_until(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            tokio::task::spawn_local(handle_connection(stream));
        }
    }));
}
