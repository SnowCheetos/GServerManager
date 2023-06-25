use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::error::Error;
use crate::utils::build::{contains_compiled_files, compile_and_install_project, run_cmake};
use crate::github::utils::{git_pull, git_diff_name_only, initialize_git_repository, add_remote_origin};
use crate::server::gunicorn::{start_gunicorn, stop_gunicorn};
use crate::server::redis::{start_redis, stop_redis};

#[derive(Clone, Debug)]
pub struct Server {
    pub name: String, // The name given to the server
    pub path: PathBuf, // Path to the server directory
    pub bind: String, // Host address assigned, default 0.0.0.0
    pub port: u32, // Port assigned, default 8000
    pub workers: u32, // Number of workers used, default 4
    pub timeout: u32, // Worker timeout value, default 30 seconds
    pub log_path: PathBuf, // Path to log file
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

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        if self.is_valid() || self.name.to_lowercase().contains("redis-server") {
            if !self.name.to_lowercase().contains("redis-server") {
                start_gunicorn(self)
            } else {
                start_redis(self)
            }
        } else {
            Err("Not a valid server directory.".into())
        }
    }    

    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        if self.running {
            if !self.name.to_lowercase().contains("redis-server") {
                stop_gunicorn(self)
            } else {
                stop_redis(self)
            }
        } else {
            Err("Server not currently running.".into())
        }
    }     

    pub fn restart(&mut self) -> Result<(), Box<dyn Error>> {
        self.stop()?;
        self.start()?;
        Ok(())
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
            let monitor_command = format!("cat {}/{}.log", self.log_path.display(), self.name);
    
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
        } else if self.name == "redis-server" {
            println!("Redis server monitoring currently unsupported.")
        } else {
            println!("Server is not currently running.")
        }
    }   
    
    pub fn clear_logs(&mut self) {
        if self.name != "redis-server" {
            let delete_command = format!("rm {}/{}.log && touch {}/{}.log", 
                self.log_path.display(), self.name,
                self.log_path.display(), self.name,
            );
            let status = Command::new("sh")
                .arg("-c")
                .arg(&delete_command)
                .status()
                .expect("Failed to remove server logs.");
        } else {
            println!("Redis server monitoring currently unsupported.")
        }
    }        
}