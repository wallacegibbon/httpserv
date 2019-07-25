use std::io::{Error, ErrorKind};
use regex::Regex;


pub fn match_query(raw: &str) -> Result<String, Error> {
    let re = match Regex::new(r"GET (?P<query>[^ ]+) HTTP") {
        Ok(regex_obj) => regex_obj,
        Err(err) => {
            let e = Error::new(ErrorKind::Other, err);
            return Err(e);
        },
    };
    let caps = match re.captures(raw) {
        Some(v) => v,
        None => {
            let e = Error::new(ErrorKind::Other, "regex capture error");
            return Err(e);
        }
    };

    Ok(caps["query"].to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_query() {
        let raw = r"GET /a/b/c/d HTTP/1.1\r\n";
        let r = match_query(raw).unwrap();
        assert_eq!(r, "/a/b/c/d");
    }
}
