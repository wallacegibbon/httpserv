use std::collections::HashMap as Map;
use std::io::Error;
use std::net::TcpStream;


type ReqHandler = fn(TcpStream) -> Result<(), Error>;

#[derive(Debug)]
pub struct Router {
    dispatcher: Map<&'static str, ReqHandler>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            dispatcher: Map::new(),
        }
    }

    pub fn bind(&mut self, url: &'static str, handler: ReqHandler) {
        self.dispatcher.insert(url, handler);
    }

    pub fn get(&self, url: &str) -> Option<&ReqHandler> {
        self.dispatcher.get(url)
    }
}
