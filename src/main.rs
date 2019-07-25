use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::fs;
use std::collections::HashMap as Map;
use httpserv::match_query;

const LISTEN_ADDR: &'static str = "0.0.0.0:7878";

fn main() {
    let listener = TcpListener::bind(LISTEN_ADDR)
        .expect(&format!("Failed listening {}", LISTEN_ADDR));

    for stream in listener.incoming() {
        handle_connection(stream.unwrap()).unwrap();
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 512];

    stream.read(&mut buffer)?;

    let first_line = String::from_utf8_lossy(&buffer[0..100]);
    let query = match_query(&first_line)?;
    println!("query: {}", query);

    if query == "/" {
        homepage(stream)
    } else {
        page404(stream)
    }
}

fn homepage(mut stream: TcpStream) -> Result<(), Error> {
    let contents = fs::read_to_string("hello.html")?;
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn page404(mut stream: TcpStream) -> Result<(), Error> {
    let contents = fs::read_to_string("404.html")?;
    let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

