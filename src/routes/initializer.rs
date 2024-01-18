use std::io::{Read, Write};
use std::net::TcpStream;
use crate::constants::api;
use crate::middlewares::middleware::middlewares;
use crate::models::http_request::HttpRequest;
use crate::routes::user::user::USER_HANDLERS;

pub fn initialize_handlers() -> Vec<(&'static str, fn(&HttpRequest) -> (String, String))> {
    let mut handlers = Vec::new();

    handlers.extend(USER_HANDLERS.iter());
    handlers
}

pub fn initialize_client(mut stream: TcpStream, handlers: &Vec<(&str, fn(&HttpRequest) -> (String, String))>) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
            let (status_line, content) = handlers.iter()
                .find_map(|(pattern, func)| {
                    if request.starts_with(pattern) {
                        Some(middlewares(func, &request))
                    } else {
                        None
                    }
                })
                .unwrap_or((api::NOT_FOUND.to_string(), "404 not found".to_string()));
            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}
