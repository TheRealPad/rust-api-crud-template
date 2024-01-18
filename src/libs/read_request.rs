use crate::models::http_method::HttpMethod;
use crate::models::http_request::HttpRequest;

pub fn parse_http_request(request_str: &str) -> Option<HttpRequest> {
    let mut lines = request_str.lines().peekable();

    if let Some(first_line) = lines.next() {
        let mut parts = first_line.split_whitespace();
        let method_str = parts.next()?;
        let endpoint = parts.next()?.to_string();
        let method = match method_str {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => return None,
        };
        let headers: Vec<(String, String)> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .filter_map(|line| {
                let mut header_parts = line.splitn(2, ':');
                let key = header_parts.next()?.trim().to_string();
                let value = header_parts.next()?.trim().to_string();
                Some((key, value))
            })
            .collect();
        let body_line = lines.collect::<Vec<&str>>().join("\n");
        let body: Vec<(String, String)> = body_line
            .split(',')
            .flat_map(|pair| {
                let mut kv_parts = pair.splitn(2, ':');
                let key = kv_parts.next()?.trim().to_string();
                let value = kv_parts
                    .next()?
                    .trim_matches(|c| c == '"' || c == '{' || c == '}')
                    .to_string();

                Some((key.replace("{", "").replace("\"", ""), value.replace("{", "").replace("}", "").replace("\"", "").trim().to_string()))
            })
            .collect();
        Some(HttpRequest {
            method,
            endpoint,
            headers,
            body,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_http_request_valid() {
        let request_str = "POST /path\nContent-Type: application/json\n\n{\"key\": \"value\"}";
        let result = parse_http_request(request_str);

        assert!(result.is_some());
        let http_request = result.unwrap();
        assert_eq!(http_request.endpoint, "/path");
        assert_eq!(http_request.headers.len(), 1);
        assert_eq!(http_request.headers[0].0, "Content-Type");
        assert_eq!(http_request.headers[0].1, "application/json");
        assert_eq!(http_request.body.len(), 1);
        assert_eq!(http_request.body[0].0, "key");
        assert_eq!(http_request.body[0].1, "value");
    }

    #[test]
    fn test_parse_http_request_invalid() {
        let request_str = "INVALID_REQUEST_STRING";
        let result = parse_http_request(request_str);

        assert!(result.is_none());
    }
}
