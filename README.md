![Rust CI](https://github.com/TheRealPad/rust-api-crud-template/actions/workflows/ci.yml/badge.svg)
![Docker deploy](https://github.com/TheRealPad/rust-api-crud-template/actions/workflows/deploy.yml/badge.svg)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Shell Script](https://img.shields.io/badge/shell_script-%23121011.svg?style=for-the-badge&logo=gnu-bash&logoColor=white)

# RUST API CRUD TEMPLATE

## How to run

### install Rust | Docker

[Rust](https://www.rust-lang.org/tools/install)

[Docker](https://docs.docker.com/get-docker/)

### run the api

Set env variable : API_PORT

Run ```cargo run```

If you want to user docker, run:
```bash
docker-compose up # add --build if you want to start fresh or with new modifications
```

## How to create a new module

1. Create new directory
2. Create the file ```mod.rs```
3. Declare the module in the direct parent of the new module (like main.rs for example)

## How to add routes

1. Create new module for the routes
2. In the module_name.rs, add a content like this
    ```rust
    lazy_static! {
        pub static ref MODULE_NAME_HANDLERS: Vec<(&'static str, fn(&HttpRequest) -> (String, String))> = {
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
    ```
3. in routes/initializer.rs, add you route handle here:
    ```rust
    pub fn initialize_handlers() -> Vec<(&'static str, fn(&HttpRequest) -> (String, String))> {
        let mut handlers = Vec::new();
    
        handlers.extend(USER_HANDLERS.iter());
        handlers
    }
    ```

## How to add controllers
1. Create a new module in ```src/controllers```
2. Create all the functions needed in this module and make them public
3. The function must be called in the functions of routes

## How to add models
1. Create a new file in ```src/models```
2. export it in ```src/models/mod.rs```
3. The content must be something like this:
    ```rust
    #[derive(Serialize, Deserialize)]
    pub struct User {
        pub id: Option<i32>,
        pub name: String,
        pub email: String,
    }
    ```

## How to add constants
1. If your constant is from the env -> go to ```src/constants/env.rs```
2. If your constant is relative to the API response -> go to ```src/constant/api.rs```
3. Else create a file in ```src/constants``` and export it in mod.rs
4. To declare the constant, do the following:
    ```rust
    pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
    ```

## How to add middlewares
1. Create new module in ```src/midllewares```
2. The middleware structure must implement ```Middleware```, like this:
    ```rust
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
    ```
3. In case of error, the middleware must throw an error with ``Err``
4. Add your middleware in ```src/middlewares/middleware.rs```
    ```rust
    pub fn middlewares(handler: &fn(&HttpRequest) -> (String, String), request: &str) -> (String, String) {
        let middlewares: Vec<Box<dyn Middleware>> = vec![
            Box::new(LoggingMiddleware), // Add your middleware here
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
    ```

## How to add libs
1. Create a new module in ```src/libs```

## How to test and run tests

### Unit test

```bash
cargo test
```

### Integration test

```bash
./tests/integration.sh
```
