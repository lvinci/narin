/// Parses an http_header from the given String.
/// Returns the name and value of the parsed header or None if the header could not be parsed.
fn parse_http_header(s: &str) -> Option<(String, String)> {
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
    use super::parse_http_header;

    #[test]
    fn http_header_parses_successfully() {
        // check whether a correct header parses correctly
        let header = parse_http_header("Content-Type: application/json");
        match header {
            Some((name, value)) => {
                assert_eq!(name, "Content-Type");
                assert_eq!(value, "application/json");
            }
            None => assert!(false),
        }
        // make sure invalid header returns None
        let result = parse_http_header("asjkdsah12321-213 21");
        assert!(result.is_none());
    }
}
