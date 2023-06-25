use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::process::{Command, Child};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::process::exit;
use crate::utils::build::{contains_compiled_files, compile_and_install_project, run_cmake};
use crate::github::utils::{git_pull, git_diff_name_only, initialize_git_repository, add_remote_origin};

#[derive(Clone, Debug)]
pub struct Server {
    pub name: String, // The name given to the server
    pub path: PathBuf, // Path to the server directory
    pub bind: String, // Host address assigned, default 0.0.0.0
    pub port: u32, // Port assigned, default 8000
    pub workers: u32, // Number of workers used, default 4
    pub timeout: u32, // Worker timeout value, default 30 seconds
    pub github: bool, // Whether or not the directory is linked to a git repository
    pub running: bool, // Whether or not the server is currently running
    pub pid: u32, // The PID of the server master worker
    pub original_dir: PathBuf // The original directory when the application was started
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
            initialize_git_repository(&self.path);
        } else {
            println!("Directory already connect to git.")
        }
    }

    pub fn git_set_origin(&mut self, remote_url: &str) {
        add_remote_origin(&self.path, remote_url);
    }

    pub fn start(&mut self) {
        // Start the server
        if self.is_valid() && self.name != "redis-server" {
            let app: String = if self.path.join("main.py").exists() {
                String::from("main:app")
            } else {
                String::from("app:app")
            };
            // navigate to self.path
            env::set_current_dir(&self.path).unwrap();
            let gunicorn_command = format!("gunicorn --workers={} --bind={}:{} --timeout={} --daemon --access-logfile gunicorn.log --error-logfile gunicorn.log {}",
                                           self.workers,
                                           self.bind,
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

            env::set_current_dir(&self.original_dir).unwrap();
        } else if self.name == "redis-server" {
            // redis-server ./configs/redis.conf
            let redis_command = if self.path.join("redis.conf").exists() {
                format!("redis-server {}/redis.conf --daemonize yes --bind {} --port {} --timeout {}", 
                    self.path.display(),
                    self.bind,
                    self.port.to_string(),
                    self.timeout.to_string()
                )
            } else {
                format!(
                    "redis-server --daemonize yes --bind {} --port {} --timeout {}",
                    self.bind,
                    self.port.to_string(),
                    self.timeout.to_string()
                )
            };
            let child: Child = Command::new("sh")
                .arg("-c")
                .arg(&redis_command)
                .spawn()
                .expect("Failed to start redis server.");
            self.pid = child.id();
            self.running = true;
        } else {
            println!("Not a valid server directory.")
        }
    }

    pub fn stop(&mut self) {
        if self.running && self.name != "redis-server" {
            // Execute the command to kill the server process
            let output = Command::new("pkill")
                .arg("-f")
                .arg(format!("gunicorn --workers={} --bind={}:{}", self.workers, self.bind, self.port))
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
        } else if self.running && self.name == "redis-server" {
            let output = Command::new("redis-cli")
                .arg("-p")
                .arg(self.port.to_string())
                .arg("shutdown")
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        self.running = false;
                        self.pid = 0;
                        println!("Stopping Redis server...");
                    } else {
                        println!("Failed to stop Redis server: {:?}", output.stderr);
                    }
                }
                Err(e) => {
                    println!("Failed to execute Redis stop command: {}", e);
                }
            }   
        } else {
            println!("Server not currently running.")
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
            let pull_result;
            {
                env::set_current_dir(&self.original_dir).unwrap();
                pull_result = git_pull(&self.path);
                env::set_current_dir(&self.original_dir).unwrap();
            }
            
            if let Err(e) = pull_result {
                println!("Failed to pull the latest changes from the Git repository: {}", e);
                return;
            }
    
            let diff_output = match git_diff_name_only("HEAD", "HEAD~1", &self.path) {
                Ok(output) => output,
                Err(e) => {
                    println!("Failed to get the diff: {}", e);
                    return;
                }
            };
    
            if contains_compiled_files(&diff_output) {
                println!("C++ source files or CMakeLists.txt have changed, rebuilding...");
    
                if diff_output.contains("CMakeLists.txt") {
                    println!("CMakeLists.txt has changed, re-running cmake...");
                    {
                        env::set_current_dir(&self.original_dir).unwrap();
                        let result = run_cmake(&self.path);
                        env::set_current_dir(&self.original_dir).unwrap();
                        if let Err(e) = result {
                            println!("Failed to run cmake: {}", e);
                            return;
                        }
                    }
                }
    
                {
                    env::set_current_dir(&self.original_dir).unwrap();
                    let result = compile_and_install_project(&self.path);
                    env::set_current_dir(&self.original_dir).unwrap();
                    if let Err(e) = result {
                        println!("Failed to compile and install the project: {}", e);
                        return;
                    }
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
        if self.running && self.name != "redis-server" {
            env::set_current_dir(&self.path).unwrap();
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
            env::set_current_dir(&self.original_dir).unwrap();
        } else if self.name == "redis-server" {
            println!("Redis server monitoring currently unsupported.")
        } else {
            println!("Server is not currently running.")
        }
    }   
    
    pub fn clear_logs(&mut self) {
        if self.name != "redis-server" {
            env::set_current_dir(&self.path).unwrap();
            let delete_command = format!("rm {} && touch {}", "gunicorn.log", "gunicorn.log");
            let status = Command::new("sh")
                .arg("-c")
                .arg(&delete_command)
                .status()
                .expect("Failed to remove server logs.");
            env::set_current_dir(&self.original_dir).unwrap();
        } else {
            println!("Redis server monitoring currently unsupported.")
        }
    }        
}