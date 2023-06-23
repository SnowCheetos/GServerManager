use std::process::Command;
use std::fs;

pub fn update_server() {
    git_pull().expect("Failed to pull from origin master");

    if is_cpp_or_cmake_changed().expect("Failed to check changes") {
        println!("C++ source files or CMakeLists.txt have changed, rebuilding...");

        if is_cmake_changed().expect("Failed to check CMakeLists.txt changes") {
            println!("CMakeLists.txt has changed, re-running cmake...");
            cmake_configure().expect("Failed to run cmake");
        }

        make().expect("Failed to run make");
    }
}

fn git_pull() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("git")
        .args(&["pull", "origin", "master"])
        .output()?;
    Ok(())
}

fn is_cpp_or_cmake_changed() -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["diff", "--name-only", "HEAD", "HEAD~1"])
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout.contains(".cpp") || stdout.contains("CMakeLists.txt"))
}

fn is_cmake_changed() -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["diff", "--name-only", "HEAD", "HEAD~1"])
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout.contains("CMakeLists.txt"))
}

fn cmake_configure() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("build")?;
    Command::new("cmake")
        .current_dir("build")
        .arg("..")
        .output()?;
    Ok(())
}

fn make() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("make")
        .current_dir("build")
        .args(&["-j4", "install"])
        .output()?;
    Ok(())
}
