use std::net::TcpListener;
use crate::constants::env;
use crate::routes::initializer::{initialize_handlers, initialize_client};

pub fn api_template() {
    let mut url: String = "0.0.0.0:".to_owned();
    url.push_str(env::API_PORT);

    let listener = TcpListener::bind(url).unwrap();
    println!("Server listening on port {}", env::API_PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                initialize_client(stream, &initialize_handlers());
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use crate::constants::api;
    use super::*;

    #[test]
    fn test_initialize_client_with_valid_request() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let mut client_stream = TcpStream::connect(addr).unwrap();
        let handlers = vec![];

        std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                initialize_client(stream, &handlers);
            }
        });
        client_stream.write_all(b"GET /users HTTP/1.1\r\n\r\n").unwrap();

        let mut response_buffer = [0; 1024];
        let response_size = client_stream.read(&mut response_buffer).unwrap();
        let response_str = String::from_utf8_lossy(&response_buffer[..response_size]).to_string();

        assert!(response_str.contains(api::NOT_FOUND));
        assert!(response_str.contains("NOT FOUND"));
    }

}
