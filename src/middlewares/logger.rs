use crate::middlewares::middleware::Middleware;
use crate::models::http_request::HttpRequest;
use chrono::{Local, DateTime, Datelike, Timelike};
use crate::models::http_method::HttpMethod;

pub struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn process(&self, request: &HttpRequest) -> Result<String, String> {
        let current_datetime: DateTime<Local> = Local::now();
        let year = current_datetime.year();
        let month = current_datetime.month();
        let day = current_datetime.day();
        let hour = current_datetime.hour();
        let minute = current_datetime.minute();
        let second = current_datetime.second();

        println!("[{}-{:02}-{:02} {:02}:{:02}:{:02}]: {:?} {}", year, month, day, hour, minute, second, request.method, request.endpoint);
        Ok("".to_string())
    }
}

#[test]
fn test_logging_middleware_process() {
    let logging_middleware = LoggingMiddleware;
    let http_request = HttpRequest {
        method: HttpMethod::POST,
        endpoint: "/path".to_string(),
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        body: vec![("key".to_string(), "value".to_string())],
    };
    let result = logging_middleware.process(&http_request);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "".to_string());
}
