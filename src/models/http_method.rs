#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_debug() {
        assert_eq!(format!("{:?}", HttpMethod::GET), "GET");
        assert_eq!(format!("{:?}", HttpMethod::POST), "POST");
        assert_eq!(format!("{:?}", HttpMethod::PUT), "PUT");
        assert_eq!(format!("{:?}", HttpMethod::DELETE), "DELETE");
    }
}
