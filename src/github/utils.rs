use std::path::Path;
use std::process::Command;

pub fn is_git_repository(path: &Path) -> bool {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .current_dir(path)
        .output()
        .expect("Failed to execute Git command.");

    output.status.success()
}

pub fn initialize_git_repository(path: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .arg("init")
        .current_dir(path)
        .output()
        .expect("Failed to execute Git command.");

    if output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        Err(error_message.into())
    }
}

pub fn add_remote_origin(path: &Path, remote_url: &str) -> Result<(), String> {
    let output = Command::new("git")
        .args(&["remote", "add", "origin", remote_url])
        .current_dir(path)
        .output()
        .expect("Failed to execute Git command.");

    if output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        Err(error_message.into())
    }
}

pub fn git_pull(path: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .args(&["pull"])
        .current_dir(path)
        .output()
        .expect("Failed to execute Git command.");

    if output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        Err(error_message.into())
    }
}

pub fn git_diff_name_only(from: &str, to: &str, path: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .args(&["diff", "--name-only", from, to])
        .current_dir(path)
        .output()
        .map_err(|e| format!("Failed to execute Git command: {}", e))?;

    if output.status.success() {
        let diff_output = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        Ok(diff_output.into())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        Err(error_message.into())
    }
}