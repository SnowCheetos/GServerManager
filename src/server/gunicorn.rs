use std::fs;
use std::env;
use std::error::Error;
use std::process::Command;
use crate::server::server::Server;

fn get_app_string(server: &mut Server, framework: &str) -> Result<String, Box<dyn Error>> {
    match framework {
        "flask" | "fastapi" => {
            if server.path.join("main.py").exists() {
                Ok(String::from("main:app"))
            } else {
                Ok(String::from("app:app"))
            }
        },
        "django" => {
            let wsgi_files = fs::read_dir(&server.path)?.filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("wsgi") {
                        Some(entry.file_name().to_string_lossy().into_owned())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }).collect::<Vec<_>>();

            if let Some(wsgi_file) = wsgi_files.first() {
                Ok(wsgi_file.clone())
            } else {
                Err("No .wsgi files found.".into())
            }
        },
        _ => Err("Unsupported framework.".into()),
    }
}

fn get_gunicorn_command(server: &mut Server, framework: &str, app: &str, absolute_log_path: &str) -> Result<String, Box<dyn Error>> {
    match framework {
        "flask" | "fastapi" => Ok(format!("gunicorn --bind={}:{} --timeout={} --daemon --access-logfile {}/{}.log --error-logfile {}/{}.log --workers={} {}",
                                            server.bind,
                                            server.port,
                                            server.timeout,
                                            absolute_log_path,
                                            server.name,
                                            absolute_log_path,
                                            server.name,
                                            server.workers,
                                            app
                                        )),
        "django" => Ok(format!("gunicorn --bind={}:{} --timeout={} --daemon --access-logfile {}/{}.log --error-logfile {}/{}.log {}",
                                            server.bind,
                                            server.port,
                                            server.timeout,
                                            absolute_log_path,
                                            server.name,
                                            absolute_log_path,
                                            server.name,
                                            app
                                        )),
        _ => Err("Unsupported framework".into()),
    }
}

pub fn start_gunicorn(server: &mut Server) -> Result<(), Box<dyn Error>> {
    let framework = server.framework.clone();
    let app = get_app_string(server, &framework)?;
    let absolute_log_path = fs::canonicalize(&server.log_path)?.to_str().ok_or("Failed to convert path to string")?.to_owned();

    // navigate to server.path
    env::set_current_dir(&server.path)?;

    let gunicorn_command = get_gunicorn_command(server, &framework, &app, &absolute_log_path)?;

    let output = Command::new("sh")
        .arg("-c")
        .arg(&gunicorn_command)
        .output()?;

    if output.status.success() {
        server.running = true;
        println!("Successfully started [{}]", server.name);
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to start [{}]: {}", server.name, error_message).into());
    }

    env::set_current_dir(&server.original_dir)?;

    Ok(())
}


pub fn stop_gunicorn(server: &mut Server) -> Result<(), Box<dyn Error>> {
    let absolute_log_path = fs::canonicalize(&server.log_path)?.to_str().ok_or("Failed to convert path to string")?.to_owned();

    //nv::set_current_dir(&server.path)?;
    let output = Command::new("pkill")
        .arg("-f")
        .arg(format!("gunicorn --bind={}:{} --timeout={} --daemon --access-logfile {}/{}.log --error-logfile {}/{}.log",
            server.bind,
            server.port,
            server.timeout,
            absolute_log_path,
            server.name,
            absolute_log_path,
            server.name
        ))
        .output()?;

    if output.status.success() {
        server.running = false;
        println!("Successfully stopped [{}]", server.name);
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to stop [{}]: {}", server.name, error_message).into());
    }
    //env::set_current_dir(&server.original_dir)?;

    Ok(())
}