use std::fs;
use std::env;
use std::error::Error;
use std::process::{Command, Child};
use crate::server::server::Server;

pub fn start_gunicorn(server: &mut Server) -> Result<(), Box<dyn Error>> {
    let app: String = if server.path.join("main.py").exists() {
        String::from("main:app")
    } else {
        String::from("app:app")
    };

    // Convert the relative log path into an absolute path and handle the error.
    let absolute_log_path = fs::canonicalize(&server.log_path)?.to_str().ok_or("Failed to convert path to string")?.to_owned();

    // navigate to server.path
    env::set_current_dir(&server.path)?;

    let gunicorn_command = format!("gunicorn --workers={} --bind={}:{} --timeout={} --daemon --access-logfile {}/{}.log --error-logfile {}/{}.log {}",
                                    server.workers,
                                    server.bind,
                                    server.port,
                                    server.timeout,
                                    absolute_log_path,
                                    server.name,
                                    absolute_log_path,
                                    server.name,
                                    app);

    // Call gunicorn with the correct options
    let child: Child = Command::new("sh")
        .arg("-c")
        .arg(&gunicorn_command)
        .spawn()?;

    // Set server.pid to the gunicorn master pid
    server.pid = child.id();
    server.running = true;

    env::set_current_dir(&server.original_dir)?;

    Ok(())
}


pub fn stop_gunicorn(server: &mut Server) -> Result<(), Box<dyn Error>> {
    env::set_current_dir(&server.path)?;
    let output = Command::new("pkill")
        .arg("-f")
        .arg(format!("gunicorn --workers={} --bind={}:{}", server.workers, server.bind, server.port))
        .output()?;

    if output.status.success() {
        server.running = false;
        server.pid = 0;
        println!("Stopping... ");
    } else {
        // Instead of just printing the error, convert it into a string and return it
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to stop server: {}", error_message).into());
    }
    env::set_current_dir(&server.original_dir)?;
    
    Ok(())
}