// Include the cli module
mod system_monitor;
mod update;
mod server;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

const REDIS_CONF_PATH: &str = "./configs/redis.conf";
const WORKERS: &str = "8";
const TIMEOUT: &str = "60";
const PORT: &str = "56009";
const ADDRESS: &str = "0.0.0.0";

fn main() {
    let mut redis_pid: Option<u32> = None;
    let mut gunicorn_pid: Option<u32> = None;
    
    env::set_var("LD_LIBRARY_PATH", "./lib:${LD_LIBRARY_PATH}");

    let mut rl = Editor::<()>::new();
    let server_op = Arc::new(Mutex::new(None));

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
                            // Replace this with the actual function to start the server
                            let (redis_pid, gunicorn_pid) = server::start_server(REDIS_CONF_PATH, WORKERS, ADDRESS, PORT, TIMEOUT);
                            println!("Server started.");
                        }));
                    },
                    "stop" => {
                        println!("Stopping server...");
                        
                        // Stop the server in a new thread
                        *server_op_lock = Some(thread::spawn(move || {
                            // Replace this with the actual function to stop the server
                            let redis_pid = redis_pid.expect("redis_pid not available");
                            let gunicorn_pid = gunicorn_pid.expect("gunicorn_pid not available");
                            server::stop_server(Some(redis_pid), Some(gunicorn_pid));
                            println!("Server stopped.");
                        }));
                    },
                    "restart" => {
                        println!("Restarting server...");
                        
                        // Restart the server in a new thread
                        *server_op_lock = Some(thread::spawn(move || {
                            // Replace this with the actual function to restart the server
                            let redis_pid = redis_pid.expect("redis_pid not available");
                            let gunicorn_pid = gunicorn_pid.expect("gunicorn_pid not available");
                            server::stop_server(Some(redis_pid), Some(gunicorn_pid));
                            let (redis_pid, gunicorn_pid) = server::start_server(REDIS_CONF_PATH, WORKERS, ADDRESS, PORT, TIMEOUT);
                            println!("Server restarted.");
                        }));
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
