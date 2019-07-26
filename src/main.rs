use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::fs;
use httpserv::{Router, ThreadPool, match_query};

const LISTEN_ADDR: &'static str = "0.0.0.0:7878";

fn main() {
    let listener = TcpListener::bind(LISTEN_ADDR)
        .expect(&format!("Failed listening {}", LISTEN_ADDR));

    let mut router = Router::new();
    router.bind("/", homepage);
    router.bind("/sleep", sleep);

    let router = Arc::new(router);

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let router = Arc::clone(&router);
        pool.execute(move || {
            handle_connection(stream.unwrap(), router)
                .expect("Failed handling connection");
        });
    }
}

fn handle_connection(mut stream: TcpStream, router: Arc<Router>)
        -> Result<(), Error> {

    let mut buffer = [0; 2048];

    stream.read(&mut buffer)?;

    let first_line = String::from_utf8_lossy(&buffer[0..100]);
    let query = match_query(&first_line)?;

    match router.get(&query) {
        Some(handler) => handler(stream),
        None => page404(stream),
    }
}

fn homepage(mut stream: TcpStream) -> Result<(), Error> {
    let contents = fs::read_to_string("hello.html")?;
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn sleep(stream: TcpStream) -> Result<(), Error> {
    thread::sleep(Duration::from_secs(5));
    homepage(stream)
}

fn page404(mut stream: TcpStream) -> Result<(), Error> {
    let contents = fs::read_to_string("404.html")?;
    let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

