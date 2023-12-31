use std::fs;
use std::env;
use std::error::Error;
use std::process::Command;
use crate::server::server::Server;

pub fn start_redis(server: &mut Server) -> Result<(), Box<dyn Error>> {
    // redis-server ./configs/redis.conf
    let absolute_log_path = fs::canonicalize(&server.log_path)?.to_str().ok_or("Failed to convert path to string")?.to_owned();
    env::set_current_dir(&server.original_dir)?;
    let redis_command = if server.path.join("redis.conf").exists() {
        format!("redis-server {}/redis.conf --daemonize yes --bind {} --port {} --timeout {} --dir ./{} --logfile {}/{}.log", 
            server.path.display(),
            server.bind,
            server.port.to_string(),
            server.timeout.to_string(),
            server.path.display(),
            absolute_log_path,
            server.name,
        )
    } else {
        format!(
            "redis-server --daemonize yes --bind {} --port {} --timeout {} --dir ./{} --logfile {}/{}.log",
            server.bind,
            server.port.to_string(),
            server.timeout.to_string(),
            server.path.display(),
            absolute_log_path,
            server.name,
        )
    };
    let output = Command::new("sh")
        .arg("-c")
        .arg(&redis_command)
        .output()?;

    if output.status.success() {
        server.running = true;
        println!("Successfully started [{}]", server.name);
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to start [{}]: {}", server.name, error_message).into());
    }

    Ok(())
}


pub fn stop_redis(server: &mut Server) -> Result<(), Box<dyn Error>> {
    env::set_current_dir(&server.original_dir)?;
    let output = Command::new("redis-cli")
                .arg("-p")
                .arg(server.port.to_string())
                .arg("shutdown")
                .output()?;
        
    if output.status.success() {
        server.running = false;
        println!("Successfully stopped [{}]", server.name);
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to stop [{}]: {}", server.name, error_message).into());
    }

    Ok(())
}