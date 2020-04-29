use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use rusteval::ThreadPool;
use http::{Response, HeaderValue, StatusCode};

const THREAD_POOL_SIZE: usize = 10;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(THREAD_POOL_SIZE);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = Response::builder()
        .header("Content-type", HeaderValue::from_static("text/plain"))
        .status(StatusCode::OK)
        .body("Hello, world!")
        .unwrap();

    let mut headers = String::from("");
    for hv in response.headers().iter() {
        headers.push_str(format!("{}: {}\r\n", hv.0.as_str(), str::from_utf8(hv.1.as_bytes()).unwrap()).as_str())
    }

    let resp_str = format!("HTTP/1.1 200 OK\r\n{}\r\n{}", headers, response.body());

    stream.write(resp_str.as_bytes()).unwrap();
    stream.flush().unwrap();
}
