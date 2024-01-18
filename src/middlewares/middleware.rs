use crate::constants::api;
use crate::libs::read_request::parse_http_request;
use crate::middlewares::logger::LoggingMiddleware;
use crate::models::http_request::HttpRequest;

pub trait Middleware {
    fn process(&self, request: &HttpRequest) -> Result<String, String>;
}

pub fn middlewares(handler: &fn(&HttpRequest) -> (String, String), request: &str) -> (String, String) {
    let middlewares: Vec<Box<dyn Middleware>> = vec![
        Box::new(LoggingMiddleware),
    ];
    let http_request = parse_http_request(request).unwrap();

    for middleware in &middlewares {
        match middleware.process(&http_request) {
            Ok(_result) => (),
            Err(err) => {
                println!("Middleware error: {}", err);
                return (api::UNAUTHORIZED.to_string(), err);
            }
        }
    }
    handler(&http_request)
}
