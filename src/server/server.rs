use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::process::{Command, Child};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::process::exit;
use crate::utils::build;
use crate::github::utils;

#[derive(Clone, Debug)]
pub struct Server {
    pub name: String, // The name given to the server
    pub path: PathBuf, // Path to the server directory
    pub host: String, // Host address assigned, default 0.0.0.0
    pub port: u32, // Port assigned, default 8000
    pub workers: u32, // Number of workers used, default 4
    pub timeout: u32, // Worker timeout value, default 30 seconds
    pub github: bool, // Whether or not the directory is linked to a git repository
    pub running: bool, // Whether or not the server is currently running
    pub pid: u32 // The PID of the server master worker
}

impl Server {
    pub fn is_valid(&self) -> bool {
        // Checks if self.path contains app.py or main.py
        if self.path.join("main.py").exists() || self.path.join("app.py").exists() {
            return true;
        } else {
            return false;
        }
    }

    pub fn git_init(&mut self) {
        if !self.github {
            utils::initialize_git_repository(&self.path);
        } else {
            println!("Directory already connect to git.")
        }
    }

    pub fn git_set_origin(&mut self, remote_url: &str) {
        utils::add_remote_origin(&self.path, remote_url);
    }

    pub fn start(&mut self) {
        // Start the server
        if self.is_valid() {
            // navigate to self.path
            env::set_current_dir(&self.path).unwrap();

            let app: String = if self.path.join("main.py").exists() {
                String::from("main:app")
            } else {
                String::from("app:app")
            };

            let gunicorn_command = format!("gunicorn --workers={} --bind={}:{} --timeout={} --daemon --access-logfile gunicorn.log --error-logfile gunicorn.log {}",
                                           self.workers,
                                           self.host,
                                           self.port,
                                           self.timeout,
                                           app);

            // Call gunicorn with the correct options
            let child: Child = Command::new("sh")
                .arg("-c")
                .arg(&gunicorn_command)
                .spawn()
                .expect("Failed to start gunicorn server.");

            // Set self.pid to the gunicorn master pid
            self.pid = child.id();

            self.running = true;
        } else {
            println!("Not a valid server directory.")
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            // Execute the command to kill the server process
            let output = Command::new("pkill")
                .arg("-f")
                .arg(format!("gunicorn --workers={} --bind={}:{}", self.workers, self.host, self.port))
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        self.running = false;
                        self.pid = 0;
                        println!("Stopping... ");
                    } else {
                        println!("Failed to stop server: {:?}", output.stderr);
                    }
                }
                Err(e) => {
                    println!("Failed to execute stop command: {}", e);
                    exit(1);
                }
            }
        } else {
            println!("Server is not currently running.");
        }
    }    

    pub fn restart(&mut self) {
        self.stop();
        self.start();
    }

    pub fn update(&mut self) {
        // Update the server
        if self.github && self.is_valid() {
            // Pull the latest changes from the Git repository
            if let Err(e) = utils::git_pull(&self.path) {
                println!("Failed to pull the latest changes from the Git repository: {}", e);
                return;
            }
    
            let diff_output = match utils::git_diff_name_only("HEAD", "HEAD~1", &self.path) {
                Ok(output) => output,
                Err(e) => {
                    println!("Failed to get the diff: {}", e);
                    return;
                }
            };
    
            if build::contains_compiled_files(&diff_output) {
                println!("C++ source files or CMakeLists.txt have changed, rebuilding...");
    
                if diff_output.contains("CMakeLists.txt") {
                    println!("CMakeLists.txt has changed, re-running cmake...");
                    if let Err(e) = build::run_cmake(&self.path) {
                        println!("Failed to run cmake: {}", e);
                        return;
                    }
                }
    
                if let Err(e) = build::compile_and_install_project(&self.path) {
                    println!("Failed to compile and install the project: {}", e);
                    return;
                }
    
                println!("Update completed successfully.");
            } else {
                println!("No C++ source files or CMakeLists.txt changes found.");
            }
        } else {
            println!("Not a valid git repository.");
        }
    }

    pub fn monitor(&self) {
        if self.running {
            let monitor_command = format!("cat {}", "gunicorn.log");
    
            let output = Command::new("sh")
                .arg("-c")
                .arg(&monitor_command)
                .output()
                .expect("Failed to retrieve server logs.");
    
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("Failed to retrieve server logs:\n{}", stderr);
            }
        } else {
            println!("Server is not currently running.")
        }
    }   
    
    pub fn clear_logs(&mut self) {
        let delete_command = format!("rm {} && touch {}", "gunicorn.log", "gunicorn.log");
        let status = Command::new("sh")
            .arg("-c")
            .arg(&delete_command)
            .status()
            .expect("Failed to remove server logs.");

    }        
}