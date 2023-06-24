// tests/server_test.rs
use GServerManager::server::server::Server;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let server = Server {
            name: String::from("test_server"),
            path: PathBuf::from("../TestServer"),
            host: String::from("0.0.0.0"),
            port: 8000,
            workers: 4,
            timeout: 30,
            github: false,
            running: false,
            pid: 0,
        };

        // Note: Make sure the server path points to a real directory containing a valid app.py or main.py file
        assert!(server.isValid());
    }

    #[test]
    fn test_start_stop() {
        let mut server = Server {
            name: String::from("test_server"),
            path: PathBuf::from("../TestServer"),
            host: String::from("0.0.0.0"),
            port: 8000,
            workers: 4,
            timeout: 30,
            github: false,
            running: false,
            pid: 0,
        };

        // Note: Make sure the server path points to a real directory containing a valid app.py or main.py file
        // And the machine should have the gunicorn installed
        server.start();
        assert!(server.running);

        server.stop();
        assert!(!server.running);
    }
}
