use std::io::{self, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use termion::{color, style};
use std::env;

use crate::commands::command::Command;
use crate::server::server::Server;
use crate::server::servers::Servers;
use crate::utils::hardware;
use crate::github::utils;


#[derive(Debug, StructOpt)]
#[structopt(name = "Server Manager", about = "Manage your servers")]
pub struct ServerManager {
    #[structopt(skip)]
    pub servers: Option<Servers>,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

impl ServerManager {
    pub fn new() -> ServerManager {
        ServerManager {
            servers: Some(Servers {
                servers: Vec::new(),
            }),
            cmd: None,
        }
    }

    pub fn execute(&mut self) {
        let _original_dir = env::current_dir().unwrap();
        match &self.cmd {
            Some(Command::Add { name, path, workers, bind, port, timeout }) => {
                if !path.exists() || !path.is_dir() {
                    println!("Invalid server path");
                    return;
                }
                if name.to_lowercase().contains("redis") {
                    println!("Name reserved for Redis, use `redis --path [path to redis config file]` instead.");
                    return;
                }
                let server = Server {
                    name: name.clone(),
                    path: path.clone(),
                    bind: bind.clone(),
                    port: *port,
                    workers: *workers,
                    timeout: *timeout,
                    github: utils::is_git_repository(path),
                    running: false,
                    pid: 0,
                    original_dir: _original_dir.to_path_buf()
                };
                if let Some(servers) = &mut self.servers {
                    match servers.add_server(server) {
                        Ok(()) => println!("Server added successfully."),
                        Err(e) => println!("Failed to add server: {}", e),
                    }
                }
            },

            Some(Command::Remove { name }) => {
                if let Some(servers) = &mut self.servers {
                    match servers.remove_server(name) {
                        Ok(()) => println!("Server removed successfully."),
                        Err(e) => println!("Failed to remove server: {}", e),
                    }
                }
            },

            Some(Command::Start { name }) => {
                if let Some(servers) = &mut self.servers {
                    match servers.start_server(name) {
                        Ok(()) => println!("Server started successfully."),
                        Err(e) => println!("Failed to start server: {}", e),
                    }
                }
            },

            Some(Command::Stop { name }) => {
                if let Some(servers) = &mut self.servers {
                    match servers.stop_server(name) {
                        Ok(()) => println!("Server stopped successfully."),
                        Err(e) => println!("Failed to stop server: {}", e),
                    }
                }
            },

            Some(Command::Restart { name }) => {
                if let Some(servers) = &mut self.servers {
                    match servers.restart_server(name) {
                        Ok(()) => println!("Server restarted successfully."),
                        Err(e) => println!("Failed to restart server: {}", e),
                    }
                }
            },

            Some(Command::List {}) => {
                if let Some(servers) = &mut self.servers {
                    servers.list_all();
                }
            },

            Some(Command::Flush {}) => {
                if let Some(servers) = &mut self.servers {
                    servers.flush();
                }
            },

            Some(Command::Monitor { name }) => {
                if let Some(servers) = &mut self.servers {
                    servers.monitor(name);
                }
            },

            Some(Command::Hardware {}) => {
                hardware::monitor_system_info();
            },

            Some(Command::GitInit { name }) => {
                if let Some(servers) = &mut self.servers {
                    servers.git_init(name);
                }
            },

            Some(Command::AddOrigin { name, remote_url }) => {
                if let Some(servers) = &mut self.servers {
                    servers.add_origin(name, remote_url);
                }
            },

            Some(Command::Update { name }) => {
                if let Some(servers) = &mut self.servers {
                    servers.update(name);
                }
            },

            Some(Command::ClearLogs { name }) => {
                if let Some(servers) = &mut self.servers {
                    servers.clear_logs(name);
                }
            },

            Some(Command::Visualize { name, show }) => {
                if name.to_lowercase().contains("redis") {
                    println!("Visualization for Redis servers not implemented.");
                    return;
                }
                if let Some(servers) = &mut self.servers {
                    servers.visualize(name, show);
                }
            },

            Some(Command::Redis { path, bind, port }) => {
                if !path.exists() || !path.is_dir() {
                    println!("Invalid redis config path");
                    return;
                }
                let mut name = String::from("redis-server");
                if let Some(servers) = &self.servers {
                    let mut counter = 1;
                    
                    // Check if the name already exists
                    while servers.name_exists(&name) {
                        counter += 1;
                        name = format!("redis-server-{}", counter);
                    }
                }
                let server = Server {
                    name: name.clone(),
                    path: path.clone(),
                    bind: bind.clone(),
                    port: *port,
                    workers: 1,
                    timeout: 30,
                    github: utils::is_git_repository(path),
                    running: false,
                    pid: 0,
                    original_dir: _original_dir.to_path_buf()
                };
                if let Some(servers) = &mut self.servers {
                    match servers.add_server(server) {
                        Ok(()) => println!("Server added successfully."),
                        Err(e) => println!("Failed to add server: {}", e),
                    }
                }
            },

            None => {
                println!("No command provided. Use --help to see available commands.");
            }
        }
    }
}
