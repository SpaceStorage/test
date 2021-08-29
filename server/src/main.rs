// vim: ts=4 sw=4 et autoindent backspace=indent,eol,start ruler showcmd
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use log::{info};
use metrics::{counter};
mod settings;

fn main() {
	let settings = settings::parse::parse_config();

    println!("Run server on addr: {}", settings.client_addr);
    println!("Run interserver communications on addr: {}", settings.internode_addr);
    let listener = TcpListener::bind(settings.client_addr).unwrap();

    // Block forever, handling each request that arrives at this IP address
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / ";

    // Respond with greetings or a 404,
    // depending on the data in the request
    print!("buffer is {}, starts_with is {}\n", String::from_utf8(buffer.to_vec()).unwrap(), String::from_utf8(get.to_vec()).unwrap());

    //let ret_status = 404;
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = "test2\n";
    info!("{}\n", filename);
    print!("{}\n", filename);
    counter!("spacestorage_count_queries", 1);

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
