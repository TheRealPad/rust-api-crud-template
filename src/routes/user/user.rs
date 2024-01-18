use lazy_static::lazy_static;
use crate::constants::api;
use crate::controllers::user::{create_user, delete_user, retrieve_user, update_user};
use crate::models::http_request::HttpRequest;

lazy_static! {
    pub static ref USER_HANDLERS: Vec<(&'static str, fn(&HttpRequest) -> (String, String))> = {
        vec![
            ("POST /users", handle_post_request),
            ("GET /users", handle_get_request),
            ("PUT /users", handle_put_request),
            ("DELETE /users", handle_delete_request),
        ]
    };
}

pub fn handle_post_request(_request: &HttpRequest) -> (String, String) {
    println!("{:?}", _request.body[0]);
    create_user::create_user();
    return (api::OK_RESPONSE.to_string(), "User created".to_string());
}

pub fn handle_get_request(_request: &HttpRequest) -> (String, String) {
    retrieve_user::retrieve_user();
    return (api::OK_RESPONSE.to_string(), "Retrieve user".to_string());
}

pub fn handle_put_request(_request: &HttpRequest) -> (String, String) {
    update_user::update_user();
    return (api::OK_RESPONSE.to_string(), "Update user".to_string());
}

pub fn handle_delete_request(_request: &HttpRequest) -> (String, String) {
    delete_user::delete_user();
    return (api::OK_RESPONSE.to_string(), "Delete user".to_string());
}

#[cfg(test)]
mod tests {
    use crate::models::http_method::HttpMethod;
    use super::*;

    #[test]
    fn test_handle_post_request() {
        let request = HttpRequest {
            method: HttpMethod::POST,
            endpoint: "/users".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: vec![("key".to_string(), "value".to_string())],
        };
        let result = handle_post_request(&request);

        assert_eq!(result, (api::OK_RESPONSE.to_string(), "User created".to_string()));
    }

    #[test]
    fn test_handle_get_request() {
        let request = HttpRequest {
            method: HttpMethod::GET,
            endpoint: "/users".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: vec![("key".to_string(), "value".to_string())],
        };
        let result = handle_get_request(&request);

        assert_eq!(result, (api::OK_RESPONSE.to_string(), "Retrieve user".to_string()));
    }

    #[test]
    fn test_handle_put_request() {
        let request = HttpRequest {
            method: HttpMethod::PUT,
            endpoint: "/users".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: vec![("key".to_string(), "value".to_string())],
        };
        let result = handle_put_request(&request);

        assert_eq!(result, (api::OK_RESPONSE.to_string(), "Update user".to_string()));
    }

    #[test]
    fn test_handle_delete_request() {
        let request = HttpRequest {
            method: HttpMethod::DELETE,
            endpoint: "/users".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: vec![("key".to_string(), "value".to_string())],
        };
        let result = handle_delete_request(&request);

        assert_eq!(result, (api::OK_RESPONSE.to_string(), "Delete user".to_string()));
    }
}
