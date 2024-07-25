mod request_parsing;
mod tcp_listener;
mod routing;


fn main() {
    tcp_listener::start_listener("127.0.0.1:8080");}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_listener(){
        tcp_listener::start_listener("127.0.0.1:8080");
    }


    #[test]
    fn test_handle_connection() {
        let mut stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
        tcp_listener::handle_connection(&mut stream);

    }

    #[test]
    fn test_invalid_address() {
        // Test case for an invalid address
        assert!(tcp_listener::start_listener("invalid_address").is_err());
    }

    #[test]
    fn test_multiple_connections() {
        // Test case for handling multiple connections
        let mut stream1 = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
        let mut stream2 = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
        tcp_listener::handle_connection(&mut stream1);
        tcp_listener::handle_connection(&mut stream2);
        // Add assertions to verify the expected behavior

        assert!(tcp_listener::handle_connection(&mut stream1).is_ok());
        assert!(tcp_listener::handle_connection(&mut stream2).is_ok());
    }

    #[test]
    fn test_handle_connection_error() {
        // Test case for handling connection errors
        let mut stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
        // Manipulate the stream to simulate an error condition
    
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        assert!(tcp_listener::handle_connection(&mut stream).is_err());
    }
}