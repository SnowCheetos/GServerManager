use crate::server::server::Server;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde::{Serialize, Deserialize};

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
        self.backup();
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
            self.backup();
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
            self.backup();
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
            self.backup();
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
            self.backup();
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
        self.backup();
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
            self.backup();
        } else {
            println!("Server not found.")
        }
    }

    pub fn git_init(&mut self, name: &str) {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].git_init();
            self.backup();
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

    pub fn backup(&self) {
        let servers_data: Vec<ServerData> = self.servers.iter().map(ServerData::from).collect();

        let json = serde_json::to_string(&servers_data).expect("Failed to serialize servers");

        let mut file = File::create("backups/servers_backup.json").expect("Failed to create backup file");
        file.write_all(json.as_bytes()).expect("Failed to write to backup file");
    }

    pub fn restore(&mut self) {
        let backup_path = Path::new("backups/servers_backup.json");

        // Check if the backup file exists
        if !backup_path.exists() {
            println!("Backup file does not exist. Assuming first launch and skipping restoration process.");
            return;
        }

        let mut file = File::open(&backup_path)
            .expect("Failed to open backup file");
        let mut json = String::new();
        file.read_to_string(&mut json).expect("Failed to read from backup file");

        let servers_data: Vec<ServerData> = serde_json::from_str(&json).expect("Failed to deserialize servers");
        self.servers = servers_data.into_iter().map(|data| data.into()).collect();
    }

}

#[derive(Serialize, Deserialize)]
struct ServerData {
    name: String,
    path: String,
    host: String,
    port: u32,
    workers: u32,
    timeout: u32,
    github: bool,
    running: bool,
    pid: u32,
    original_dir: PathBuf
}

impl From<&Server> for ServerData {
    fn from(server: &Server) -> Self {
        Self {
            name: server.name.clone(),
            path: server.path.to_str().unwrap().to_string(),
            host: server.host.clone(),
            port: server.port,
            workers: server.workers,
            timeout: server.timeout,
            github: server.github,
            running: server.running,
            pid: server.pid,
            original_dir: server.original_dir.clone()
        }
    }
}

impl Into<Server> for ServerData {
    fn into(self) -> Server {
        Server {
            name: self.name,
            path: PathBuf::from(self.path),
            host: self.host,
            port: self.port,
            workers: self.workers,
            timeout: self.timeout,
            github: self.github,
            running: self.running,
            pid: self.pid,
            original_dir: self.original_dir
        }
    }
}