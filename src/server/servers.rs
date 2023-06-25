use crate::server::server::Server;
use std::path::Path;
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde::{Serialize, Deserialize};
use std::process::Command;


#[derive(Debug)]
pub struct Servers {
    pub servers: Vec<Server>,
}

impl Servers {
    pub fn num_servers(&self) -> usize {
        self.servers.len()
    }

    pub fn list_all(&mut self) {
        println!("[INFO] Listing all available servers");
        println!("[INFO] [*]: Running | [ ]: Not running \n");
        for server in &mut self.servers {
            let symbol = if server.running {
                String::from("*")
            } else {
                String::from(" ")
            };
            println!("[{}] Name: {} | Address: {}:{} | Workers: {} | Timeout: {}s | Log Path: {} |", 
                symbol,
                server.name, 
                server.bind, 
                server.port, 
                server.workers,
                server.timeout,
                server.log_path.display()
            );
        }
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

        let new_server_name = new_server.name.clone();
        self.servers.push(new_server);
        self.backup();
        println!("Successfully added [{}]", new_server_name);
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
            println!("Successfully removed [{}]", name);
            self.backup();
            Ok(())
        } else {
            Err(String::from("Server not found"))
        }
    }

    pub fn start_server(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let index = self.servers.iter().position(|s| s.name == name);
    
        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].start()?;
            self.backup();
            Ok(())
        } else {
            Err("Server not found".into())
        }
    }    

    pub fn stop_server(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            // Safely shut down the server before removing
            self.servers[index].stop()?;
            self.backup();
            Ok(())
        } else {
            Err("Server not found".into())
        }
    }

    pub fn restart_server(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            self.servers[index].restart()?;
            self.backup();
            Ok(())
        } else {
            Err("Server not found".into())
        }
    }

    pub fn flush(&mut self) {
        let mut errors = vec![];
        let mut stopped_indices = Vec::new();
    
        for (index, server) in self.servers.iter_mut().enumerate() {
            match server.stop() {
                Ok(_) => stopped_indices.push(index),
                Err(e) => errors.push(format!("Failed to stop server {}: {}", server.name, e)),
            }
        }
    
        // Remove the successfully stopped servers in reverse order
        for i in stopped_indices.into_iter().rev() {
            self.servers.remove(i);
        }
    
        self.backup();
    
        // Print out any errors that occurred
        if !errors.is_empty() {
            for error in &errors {
                eprintln!("{}", error);
            }
        }
    }

    pub fn monitor(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            self.servers[index].monitor()?;
            Ok(())
        } else {
            Err("Server not found".into())
        }
    }

    pub fn clear_logs(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
            self.servers[index].clear_logs()?;
            Ok(())
        } else {
            Err("Server not found".into())
        }
    }


    ////////////////////////////////////
    pub fn update(&mut self, name: &str) {
        let index = self.servers.iter().position(|s| s.name == name);

        if let Some(index) = index {
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

    // Helper function to check if a server name already exists
    pub fn name_exists(&self, name: &str) -> bool {
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
            println!("No back up file found. Skipping restoration process.");
            return;
        }

        let mut file = File::open(&backup_path)
            .expect("Failed to open backup file");
        let mut json = String::new();
        file.read_to_string(&mut json).expect("Failed to read from backup file");

        let servers_data: Vec<ServerData> = serde_json::from_str(&json).expect("Failed to deserialize servers");
        self.servers = servers_data.into_iter().map(|data| data.into()).collect();
    }

    pub fn visualize(&self, name: &str, show: &bool) {
        let index = self.servers.iter().position(|s| s.name == name);
        let show_arg = if *show {
            String::from("True")
        } else {
            String::from("False")
        };
        if let Some(index) = index {
            let log_path = self.servers[index].path.join("server.log");
            if log_path.exists() {
                let output = Command::new("python")
                .arg("scripts/main.py")
                .arg(log_path)
                .arg(show_arg)
                .output()
                .expect("Failed to execute Python script");
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
            
                    println!("Python script executed successfully:");
                    println!("Output:{}", stdout);
                    println!("Errors:{}", stderr);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute Python script:");
                    eprintln!("{}", stderr);
                }
            } else {
                println!("Log file unavailable.")
            }
        } else {
            println!("Server not found.")
        }
    }

}

#[derive(Serialize, Deserialize)]
struct ServerData {
    name: String,
    path: String,
    bind: String,
    port: u32,
    workers: u32,
    timeout: u32,
    log_path: PathBuf,
    github: bool,
    running: bool,
    framework: String,
    original_dir: PathBuf
}

impl From<&Server> for ServerData {
    fn from(server: &Server) -> Self {
        Self {
            name: server.name.clone(),
            path: server.path.to_str().unwrap().to_string(),
            bind: server.bind.clone(),
            port: server.port,
            workers: server.workers,
            timeout: server.timeout,
            log_path: server.log_path.to_str().unwrap().into(),
            github: server.github,
            running: server.running,
            framework: server.framework.clone(),
            original_dir: server.original_dir.clone()
        }
    }
}

impl Into<Server> for ServerData {
    fn into(self) -> Server {
        Server {
            name: self.name,
            path: PathBuf::from(self.path),
            bind: self.bind,
            port: self.port,
            workers: self.workers,
            timeout: self.timeout,
            log_path: PathBuf::from(self.log_path),
            github: self.github,
            running: self.running,
            framework: self.framework,
            original_dir: self.original_dir
        }
    }
}