pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

pub fn parse_request(buffer: &[u8]) -> HttpRequest {
    let request = String::from_utf8_lossy(buffer);

    let mut lines = request.lines();
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap().to_string();
    let path = parts.next().unwrap().to_string();

    let mut headers = Vec::new();
    for line in request.lines() {

        let mut header_parts = line.split(": ");
        let name = match header_parts.next() {
        Some(name) => name.to_string(),
        None => continue, // Skip this iteration if there's no name
        };

        let value = match header_parts.next() {
            Some(value) => value.to_string(),
            None => continue, // Skip this iteration if there's no value
        };
        headers.push((name, value));

        }
    let body = request.lines().collect::<Vec<&str>>().join("\n");

    HttpRequest {
        method,
        path,
        headers,
        body,
    } 
}