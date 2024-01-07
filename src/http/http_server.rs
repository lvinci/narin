use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::http::http_request::parse_http_request;

pub fn start_http_server(port: u16) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    match parse_http_request(&mut buf_reader) {
        Ok(request) => {
            println!("{}", request.path);
            for (name, value) in request.headers {
                println!("{}={}", name, value);
            }
            println!("{}", request.body);
            stream
                .write_all(
                    "HTTP/1.1 200 OK\nContent-Length: 2\n\nok"
                        .to_string()
                        .as_bytes(),
                )
                .unwrap();
        }
        Err(_) => println!("http request parse error"),
    };
}
