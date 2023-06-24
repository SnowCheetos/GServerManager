use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::process::{Command, Child};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use signal_hook::consts::signal::*;
use signal_hook::flag;


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
    pub pid: u32, // The PID of the server master worker
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
        // Stop the server
        if self.running {
            kill(Pid::from_raw(self.pid as i32), Signal::SIGTERM).unwrap();
            self.running = false;
        } else {
            println!("Server is not currently running.")
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
            let status = Command::new("git")
                .args(&["pull", "origin", "master"])
                .status()
                .expect("Failed to pull the latest changes from the Git repository.");

            if status.success() {
                let output = Command::new("git")
                    .args(&["diff", "--name-only", "HEAD", "HEAD~1"])
                    .output()
                    .expect("Failed to get the diff.");

                let output_str = String::from_utf8(output.stdout).unwrap();

                // Check if any C++ source files or the CMakeLists.txt file has changed
                if output_str.contains("CMakeLists.txt") || output_str.contains("src") {
                    println!("C++ source files or CMakeLists.txt have changed, rebuilding...");

                    // Check if the CMake file has changed
                    if output_str.contains("CMakeLists.txt") {
                        println!("CMakeLists.txt has changed, re-running cmake...");
                        
                        // navigate to build directory
                        env::set_current_dir(Path::new(&self.path).join("build")).unwrap();

                        Command::new("cmake")
                            .arg("..")
                            .status()
                            .expect("Failed to run cmake.");

                        // return to the original directory
                        env::set_current_dir(&self.path).unwrap();
                    }

                    // Compile the project
                    env::set_current_dir(Path::new(&self.path).join("build")).unwrap();

                    Command::new("make")
                        .arg("-j4")
                        .status()
                        .expect("Failed to run make.");

                    Command::new("make")
                        .arg("install")
                        .status()
                        .expect("Failed to run make install.");

                    // return to the original directory
                    env::set_current_dir(&self.path).unwrap();
                }
            }
        } else {
            println!("Not a valid git repository.");
        }
    }

    pub fn monitor(&self) {
        let term = Arc::new(AtomicBool::new(false));
        flag::register_conditional_shutdown(SIGINT, 0, Arc::clone(&term)).unwrap();

        if self.running {
            let monitor_command = format!("tail -f {}", "gunicorn.log");

            Command::new("sh")
                .arg("-c")
                .arg(&monitor_command)
                .spawn()
                .expect("Failed to monitor the server.");

            println!("Monitoring server... Press Ctrl+C to stop.");

            term.store(true, Ordering::Relaxed);
        } else {
            println!("Server is not currently running.")
        }
    }
}