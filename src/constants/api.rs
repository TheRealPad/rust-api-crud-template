pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
pub const UNAUTHORIZED: &str = "HTTP/1.1 401 UNAUTHORIZED\r\n\r\n";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_response() {
        assert_eq!(
            OK_RESPONSE,
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n"
        );
    }

    #[test]
    fn test_not_found() {
        assert_eq!(NOT_FOUND, "HTTP/1.1 404 NOT FOUND\r\n\r\n");
    }

    #[test]
    fn test_unauthorized() {
        assert_eq!(UNAUTHORIZED, "HTTP/1.1 401 UNAUTHORIZED\r\n\r\n");
    }
}
