use crate::server::server::Server;
use std::path::Path;


#[derive(Debug)]
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

    pub fn start_server(&mut self, name: &str) -> Result<(), String> {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].start();

            Ok(())
        } else {
            Err(String::from("Server not found"))
        }
    }

    pub fn stop_server(&mut self, name: &str) -> Result<(), String> {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].stop();

            Ok(())
        } else {
            Err(String::from("Server not found"))
        }
    }

    pub fn restart_server(&mut self, name: &str) -> Result<(), String> {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].restart();

            Ok(())
        } else {
            Err(String::from("Server not found"))
        }
    }

    pub fn flush(&mut self) {
        for server in &mut self.servers {
            server.stop();
        }
        self.servers.clear();
    }

    pub fn monitor(&mut self, name: &str) {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].monitor();
        } else {
            println!("Server not found.")
        }
    }

    pub fn update(&mut self, name: &str) {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].update();
        } else {
            println!("Server not found.")
        }
    }

    pub fn git_init(&mut self, name: &str) {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].git_init();
        } else {
            println!("Server not found.")
        }
    }

    pub fn add_origin(&mut self, name: &str, remote_url: &str) {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].git_set_origin(remote_url);
        } else {
            println!("Server not found.")
        }
    }

    pub fn list_all(&mut self) {
        println!("[INFO] Listing all available servers");
        println!("[INFO] [*]: Running | [ ]: Not running \n");
        for server in &mut self.servers {
            if server.running {
                println!("[*] Name: {} | Address: {}:{} | Workers: {} | Timeout: {}s | PID: {} |", 
                    server.name, 
                    server.host, 
                    server.port, 
                    server.workers,
                    server.timeout,
                    server.pid
                );
            } else {
                println!("[ ] Name: {} | Address: {}:{} | Workers: {} | Timeout: {}s | PID: {} |", 
                    server.name, 
                    server.host, 
                    server.port, 
                    server.workers,
                    server.timeout,
                    server.pid
                );
            }
        }
    }

    pub fn clear_logs(&mut self, name: &str) {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].clear_logs();
        } else {
            println!("Server not found.")
        }
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
    fn port_exists(&self, port: u32) -> bool {
        self.servers.iter().any(|s| s.port == port)
    }
}