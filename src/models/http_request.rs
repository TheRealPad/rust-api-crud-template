use crate::models::http_method::HttpMethod;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub endpoint: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_debug() {
        let http_request = HttpRequest {
            method: HttpMethod::GET,
            endpoint: "/path".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: vec![("key".to_string(), "value".to_string())],
        };
        let debug_str = format!("{:?}", http_request);

        assert!(debug_str.contains("method: GET"));
        assert!(debug_str.contains("endpoint: \"/path\""));
        assert!(debug_str.contains("headers: [(\"Content-Type\", \"application/json\")]"));
        assert!(debug_str.contains("body: [(\"key\", \"value\")]"));
    }
}
