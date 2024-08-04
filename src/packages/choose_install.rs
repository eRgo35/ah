use colored::Colorize;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::packages::PACKAGE_MANAGER;

pub fn choose_install(query: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "::".bold().green(),
        "Looking for package...".bold()
    );

    if query.is_empty() {
        return Err("No query provided".into());
    }

    let mut child = Command::new(PACKAGE_MANAGER)
        .arg("--color")
        .arg("always")
        .arg("s")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    if let Some(mut stdin) = child.stdin.take() {
        for word in query.clone() {
            writeln!(stdin, "{}", word).unwrap();
        }
    }

    let status = child.wait().expect("Failed to wait on child");

    if !status.success() {
        return Err("Failed to install packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages installed".bold());

    println!(
        "{} {}",
        "::".bold().red(),
        "Package index has not been updated!".bold()
    );

    Ok(())
}
