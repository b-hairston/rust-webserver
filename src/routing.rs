use crate::request_parsing::HttpRequest;

pub fn route_request(request: &HttpRequest) -> String {
    match request.path.as_str(){
        "/" => handle_root(),
        "/hello" => handle_hello(),
        "/bye" => handle_bye(),
        _ => handle_404(),
    }
}

fn handle_root() -> String {
    format!("HTTP/1.1 200 OK\r\n\r\n{}", "Welcome to the home page")
}

fn handle_hello() -> String {
    format!("HTTP/1.1 200 OK\r\n\r\n{}", "Hello, World!")
}

fn handle_404() -> String {
    format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", "Not Found")
}

fn handle_bye() -> String {
    format!("HTTP/1.1 200 OK\r\n\r\n{}", "Goodbye!")
    
}