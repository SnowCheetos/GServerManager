// Include the cli module
mod system_monitor;
mod update;
mod server;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
use std::fs;
use std::io::{self, Write};

const REDIS_CONF_PATH: &str = "./configs/redis.conf";
const WORKERS: &str = "8";
const TIMEOUT: &str = "60";
const PORT: &str = "56009";
const ADDRESS: &str = "0.0.0.0";

fn main() {
    let scripts_dir = env::current_dir()
        .expect("Failed to retrieve current working directory")
        .join("scripts");

    let default_dir = loop {
        println!("Please enter the relative path to the desired folder:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read user input");
        let folder_path = scripts_dir.join(input.trim());
        if folder_path.is_dir() {
            break folder_path;
        } else {
            println!("Invalid directory. Please try again.");
        }
    };
    
    env::set_current_dir(&default_dir).expect("Failed to set default directory");
    env::set_var("LD_LIBRARY_PATH", "./lib:${LD_LIBRARY_PATH}");

    let mut redis_pid: Option<u32> = None;
    let mut gunicorn_pid: Option<u32> = None;

    let mut rl = Editor::<()>::new();
    let server_op = Arc::new(Mutex::new(None));

    // let mut redis_conf_path = REDIS_CONF_PATH.to_string();
    // let mut workers = WORKERS.to_string();
    // let mut address = ADDRESS.to_string();
    // let mut timeout = TIMEOUT.to_string();
    // let mut port = PORT.to_string();

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let server_op = Arc::clone(&server_op);
                let mut server_op_lock = server_op.lock().unwrap();
                match line.as_str() {
                    "start" => {
                        println!("Starting server...");
                        *server_op_lock = Some(thread::spawn(move || {
                            if let Ok((r_pid, g_pid)) = server::start_server(REDIS_CONF_PATH, WORKERS, ADDRESS, PORT, TIMEOUT) {
                                println!(
                                    "Server started with Redis PID: {}, Gunicorn PID: {}",
                                    r_pid.unwrap_or(0),
                                    g_pid.unwrap_or(0)
                                );
                                // Assign the new PIDs to redis_pid and gunicorn_pid
                                redis_pid = r_pid;
                                gunicorn_pid = g_pid;
                            } else {
                                println!("Error starting the server.");
                            }
                        }));
                    },
                    
                    "stop" => {
                        println!("Stopping server...");
                    
                        if redis_pid.is_some() && gunicorn_pid.is_some() {
                            *server_op_lock = Some(thread::spawn(move || {
                                server::stop_server(redis_pid, gunicorn_pid);
                                println!("Server stopped.");
                            }));
                        } else {
                            println!("Redis PID or Gunicorn PID is not available.");
                        }
                    },
                    
                    "restart" => {
                        println!("Restarting server...");
                        println!("Stopping server...");
                    
                        if redis_pid.is_some() && gunicorn_pid.is_some() {
                            *server_op_lock = Some(thread::spawn(move || {
                                server::stop_server(redis_pid, gunicorn_pid);
                                println!("Server stopped.");
                            }));
                        } else {
                            println!("Redis PID or Gunicorn PID is not available.");
                        }

                        "start" => {
                            println!("Starting server...");
                            *server_op_lock = Some(thread::spawn(move || {
                                if let Ok((r_pid, g_pid)) = server::start_server(REDIS_CONF_PATH, WORKERS, ADDRESS, PORT, TIMEOUT) {
                                    println!(
                                        "Server started with Redis PID: {}, Gunicorn PID: {}",
                                        r_pid.unwrap_or(0),
                                        g_pid.unwrap_or(0)
                                    );
                                    // Assign the new PIDs to redis_pid and gunicorn_pid
                                    redis_pid = r_pid;
                                    gunicorn_pid = g_pid;
                                } else {
                                    println!("Error starting the server.");
                                }
                            }));
                        },                        

                    },
                    "update" => {
                        println!("Updating server...");
                        
                        // Update the server in a new thread
                        *server_op_lock = Some(thread::spawn(move || {
                            update::update_server();
                        }));
                    },
                    "sysinfo" => {
                        // prints system usage
                        system_monitor::print_system_info();
                    }
                    "exit" => {
                        // Make sure the server operation is finished before exiting
                        if let Some(thread) = server_op_lock.take() {
                            thread.join().unwrap();
                        }
                        
                        break;
                    },
                    _ => {
                        println!("Unknown command: {}", line);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
