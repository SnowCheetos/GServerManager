use std::path::Path;
use std::process::exit;
use std::process::Command;
use std::process::Child;

pub fn start_server(redis_conf_path: &str, workers: &str, address: &str, port: &str, timeout: &str) -> Result<(Option<u32>, Option<u32>), String> {
    // Check if the Redis configuration file exists
    if !Path::new(redis_conf_path).exists() {
        return Err(format!("Redis configuration file does not exist at path {}", redis_conf_path));
    }

    // Start the Redis server with a custom configuration file
    let redis_server = Command::new("redis-server")
        .arg(redis_conf_path)
        .spawn();

    let redis_pid = match redis_server {
        Ok(child) => Some(child.id()),
        Err(e) => {
            println!("Failed to start Redis server: {}", e);
            exit(1);
        }
    };

    // Start the Python Flask server
    let gunicorn_server = Command::new("gunicorn")
        .arg("--workers")
        .arg(workers)
        .arg("--bind")
        .arg(format!("{}:{}", address, port))
        .arg("main:app")
        .arg("--timeout")
        .arg(timeout)
        .spawn();

    let gunicorn_pid = match gunicorn_server {
        Ok(child) => Some(child.id()),
        Err(e) => {
            return Err(format!("Failed to start Gunicorn server: {}", e));
        }
    };        

    Ok((redis_pid, gunicorn_pid))
}

pub fn stop_server(redis_pid: Option<u32>, gunicorn_pid: Option<u32>) {
    // Terminate the Redis server process
    if let Some(pid) = redis_pid {
        let redis_stop = Command::new("kill")
            .arg(pid.to_string())
            .output();

        match redis_stop {
            Ok(_) => println!("Redis server stopped."),
            Err(e) => println!("Failed to stop Redis server: {}", e),
        }
    }

    // Terminate the Gunicorn server process
    if let Some(pid) = gunicorn_pid {
        let gunicorn_stop = Command::new("kill")
            .arg(pid.to_string())
            .output();

        match gunicorn_stop {
            Ok(_) => println!("Gunicorn server stopped."),
            Err(e) => println!("Failed to stop Gunicorn server: {}", e),
        }
    }
}