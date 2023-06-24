use GServerManager::server::server::Server;
use GServerManager::server::servers::Servers;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_servers_functionality() {
        // Create a new Servers instance
        let mut servers = Servers {
            servers: Vec::new(),
        };

        // Create a test server
        let test_server = Server {
            name: String::from("test-server"),
            path: PathBuf::from("../TestServer"),
            host: String::from("0.0.0.0"),
            port: 8000,
            workers: 4,
            timeout: 30,
            github: false,
            running: false,
            pid: 0,
        };

        // Add the test server to the servers list
        servers.add_server(test_server.clone()).unwrap();

        // Check the number of servers
        assert_eq!(servers.num_servers(), 1);

        // Fetch the test server by name
        let fetched_server = servers.fetch_server("test-server");
        assert!(fetched_server.is_some());
        assert_eq!(fetched_server.unwrap().name, "test-server");

        // Attempt to add a server with the same name (should fail)
        let duplicate_name_server = Server {
            name: String::from("test-server"),
            path: PathBuf::from("../TestServer"),
            host: String::from("0.0.0.0"),
            port: 8001,
            workers: 4,
            timeout: 30,
            github: false,
            running: false,
            pid: 0,
        };
        assert!(servers.add_server(duplicate_name_server).is_err());

        // Attempt to add a server with the same path (should fail)
        let duplicate_path_server = Server {
            name: String::from("another-server"),
            path: PathBuf::from("../TestServer"),
            host: String::from("0.0.0.0"),
            port: 8001,
            workers: 4,
            timeout: 30,
            github: false,
            running: false,
            pid: 0,
        };
        assert!(servers.add_server(duplicate_path_server).is_err());

        // Attempt to add a server with the same port (should fail)
        let duplicate_port_server = Server {
            name: String::from("another-server"),
            path: PathBuf::from("../TestServer"),
            host: String::from("0.0.0.0"),
            port: 8000,
            workers: 4,
            timeout: 30,
            github: false,
            running: false,
            pid: 0,
        };
        assert!(servers.add_server(duplicate_port_server).is_err());

        // Remove the test server
        servers.remove_server("test-server").unwrap();

        // Check the number of servers after removal
        assert_eq!(servers.num_servers(), 0);

        // Attempt to remove a non-existent server (should fail)
        assert!(servers.remove_server("non-existent-server").is_err());

        // Flush the servers
        servers.flush();

        // Check the number of servers after flushing
        assert_eq!(servers.num_servers(), 0);
    }
}