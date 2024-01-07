use std::{collections::HashMap, io::BufRead};

pub enum HttpParseError {
    ParseStartLineError,
    ParseHeaderError,
    ReadBodyError,
}

pub struct HttpRequest {
    pub method: HttpRequestMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub fn parse_http_request<R: BufRead>(reader: &mut R) -> Result<HttpRequest, HttpParseError> {
    let mut line = String::new();
    if reader.read_line(&mut line).is_err() {
        return Err(HttpParseError::ParseStartLineError);
    }
    let (method, path) = parse_http_start_line(line);
    let mut headers: HashMap<String, String> = HashMap::new();

    let raw_headers: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    for raw_header in raw_headers {
        match parse_http_header(raw_header) {
            Some((name, value)) => {
                headers.insert(name, value);
            }
            None => return Err(HttpParseError::ParseHeaderError),
        }
    }

    let content_length: i32 = match headers.get("Content-Length") {
        Some(header) => header.parse().unwrap_or(0),
        None => 0,
    };

    let mut body_buffer: Vec<u8> = vec![0, content_length as u8];

    if reader.read_to_end(&mut body_buffer).is_err() {
        return Err(HttpParseError::ReadBodyError);
    }

    let body = String::from_utf8(body_buffer)
        .unwrap_or(String::new())
        .replace("\0\u{f}", "");

    return Ok(HttpRequest {
        method: method,
        path: path,
        headers: headers,
        body: body,
    });
}

/// HTTP defines a set of request methods to indicate the desired action to be performed for a given resource.
/// Although they can also be nouns, these request methods are sometimes referred to as HTTP verbs.
/// Each of them implements a different semantic, but some common features are shared by a group of them:
/// e.g. a request method can be safe, idempotent, or cacheable.
/// source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(Debug, PartialEq)]
pub enum HttpRequestMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

fn http_request_method_from_string(s: &str) -> HttpRequestMethod {
    return match s.to_uppercase().as_str() {
        "GET" => HttpRequestMethod::Get,
        "HEAD" => HttpRequestMethod::Head,
        "POST" => HttpRequestMethod::Post,
        "PUT" => HttpRequestMethod::Put,
        "DELETE" => HttpRequestMethod::Delete,
        "CONNECT" => HttpRequestMethod::Connect,
        "OPTIONS" => HttpRequestMethod::Options,
        "TRACE" => HttpRequestMethod::Trace,
        "Patch" => HttpRequestMethod::Patch,
        _ => HttpRequestMethod::Get,
    };
}

fn parse_http_start_line(s: String) -> (HttpRequestMethod, String) {
    let mut parts = s.split(" ");
    let method = match parts.next() {
        Some(method_str) => http_request_method_from_string(method_str),
        None => HttpRequestMethod::Get,
    };
    let path = match parts.next() {
        Some(path) => path,
        None => "/",
    };
    return (method, path.to_string());
}

/// Parses an http_header from the given String.
/// Returns the name and value of the parsed header or None if the header could not be parsed.
fn parse_http_header(s: String) -> Option<(String, String)> {
    let mut parts = s.split(": ");
    let name: String;
    let value: String;
    match parts.next() {
        Some(part) => name = part.to_string(),
        None => return None,
    }
    match parts.next() {
        Some(part) => value = part.to_string(),
        None => return None,
    }
    return Some((name, value));
}

#[cfg(test)]
mod tests {

    use crate::http::http_request::{parse_http_request, HttpRequestMethod};

    use super::{parse_http_header, parse_http_start_line};

    #[test]
    fn http_start_line_parses_successfully() {
        let (method, path) =
            parse_http_start_line("GET /api/index.html?a=1&b=2 HTTP/1.1".to_string());
        assert_eq!(method, HttpRequestMethod::Get);
        assert_eq!(path, "/api/index.html?a=1&b=2");
    }

    #[test]
    fn http_header_parses_successfully() {
        // check whether a correct header parses correctly
        let header = parse_http_header("Content-Type: application/json".to_string());
        match header {
            Some((name, value)) => {
                assert_eq!(name, "Content-Type");
                assert_eq!(value, "application/json");
            }
            None => assert!(false),
        }
        // make sure invalid header returns None
        let result = parse_http_header("asjkdsah12321-213 21".to_string());
        assert!(result.is_none());
    }

    #[test]
    fn http_request_parses_successfully() {
        const RAW_REQUEST: &str = "DELETE /users/actor HTTP/1.1\nContent-Type: application/json\nContent-Length: 15\n\n{\"proceed\": true}";
        match parse_http_request(&mut RAW_REQUEST.as_bytes()) {
            Ok(request) => {
                assert_eq!(request.method, HttpRequestMethod::Delete);
                assert_eq!(request.path, "/users/actor");
                assert_eq!(request.headers.len(), 2);
                assert_eq!(request.headers["Content-Type"], "application/json");
                assert_eq!(request.headers["Content-Length"], "15");
                assert_eq!(request.body, "{\"proceed\": true}");
            }
            Err(_) => assert!(false),
        };
    }
}
