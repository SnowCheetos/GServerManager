use std::io::{self, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use termion::{color, style};

use crate::commands::command::Command;
use crate::server::server::Server;
use crate::server::servers::Servers;
use crate::utils::hardware;

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
        match &self.cmd {
            Some(Command::Add { name, path, workers, host, port, timeout, log_file }) => {
                let server = Server {
                    name: name.clone(),
                    path: path.clone(),
                    host: host.clone(),
                    port: *port,
                    workers: *workers,
                    timeout: *timeout,
                    github: false,
                    running: false,
                    pid: 0,
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

            _ => {
                println!("No command provided. Use --help to see available commands.");
                // You can choose to exit gracefully or continue the program flow here
            }
        }
    }
}
