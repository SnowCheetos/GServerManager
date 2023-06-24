use crate::server::server::Server;
use std::path::Path;


pub struct Servers {
    pub servers: Vec<Server>,
}

impl Servers {
    pub fn num_servers(&self) -> usize {
        self.servers.len()
    }

    pub fn add_server(&mut self, new_server: Server) -> Result<(), String> {
        if self.name_exists(&new_server.name) {
            return Err(String::from("Server name already exists"));
        }

        if self.path_exists(&new_server.path) {
            return Err(String::from("Server path already exists"));
        }

        if self.port_exists(new_server.port) {
            return Err(String::from("Server port already exists"));
        }

        self.servers.push(new_server);
        Ok(())
    }

    pub fn fetch_server(&self, name: &str) -> Option<&Server> {
        self.servers.iter().find(|s| s.name == name)
    }

    pub fn remove_server(&mut self, name: &str) -> Result<(), String> {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].stop();

            self.servers.remove(index);
            Ok(())
        } else {
            Err(String::from("Server not found"))
        }
    }

    pub fn flush(&mut self) {
        self.servers.clear();
    }

    // Helper function to check if a server name already exists
    fn name_exists(&self, name: &str) -> bool {
        self.servers.iter().any(|s| s.name == name)
    }

    // Helper function to check if a server path already exists
    fn path_exists(&self, path: &Path) -> bool {
        self.servers.iter().any(|s| s.path == path)
    }

    // Helper function to check if a server port already exists
    fn port_exists(&self, port: i32) -> bool {
        self.servers.iter().any(|s| s.port == port)
    }
}