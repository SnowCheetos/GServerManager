use std::env;
use std::path::Path;
use std::process::Command;

pub fn contains_compiled_files(diff_output: &str) -> bool {
    diff_output.contains("CMakeLists.txt") || diff_output.contains("src")
}

pub fn run_cmake(path: &Path) -> Result<(), String> {
    env::set_current_dir(Path::new(path).join("build"))
        .map_err(|e| format!("Failed to navigate to build directory: {}", e))?;

    Command::new("cmake")
        .arg("..")
        .status()
        .map_err(|e| format!("Failed to run cmake: {}", e))?;

    env::set_current_dir(path)
        .map_err(|e| format!("Failed to return to the original directory: {}", e))?;

    Ok(())
}

pub fn compile_and_install_project(path: &Path) -> Result<(), String> {
    env::set_current_dir(Path::new(path).join("build"))
        .map_err(|e| format!("Failed to navigate to build directory: {}", e))?;

    Command::new("make")
        .arg("-j4")
        .status()
        .map_err(|e| format!("Failed to run make: {}", e))?;

    Command::new("make")
        .arg("install")
        .status()
        .map_err(|e| format!("Failed to run make install: {}", e))?;

    env::set_current_dir(path)
        .map_err(|e| format!("Failed to return to the original directory: {}", e))?;

    Ok(())
}