use std::net::TcpListener;
use std::io::{Read, Write};
use crate::request_parsing::parse_request;
use crate::routing::route_request;

pub fn start_listener(address: &str){
    let listener = TcpListener::bind(address).unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        handle_connection(&mut stream);
    }
}

fn handle_connection(stream: &mut std::net::TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let request = parse_request(&buffer);
    let response = route_request(&request);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}