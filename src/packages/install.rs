use crate::{file, packages::get_package_path};
use colored::Colorize;
use std::io::Write;
use std::process::{Command, Stdio};

const PACKAGE_MANAGER: &str = "paru";

pub fn install(new_packages: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "::".bold().green(),
        "Installing packages...".bold()
    );

    let packages = file::read_packages(get_package_path());

    let packages = packages
        .into_iter()
        .filter(|p| !p.contains("#") && !p.is_empty())
        .collect::<Vec<String>>();

    let mut child = Command::new(PACKAGE_MANAGER)
        .arg("--color")
        .arg("always")
        .arg("-S")
        .arg("--needed")
        // .arg(noconfirm)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    if let Some(mut stdin) = child.stdin.take() {
        for new_package in new_packages.clone() {
            writeln!(stdin, "{}", new_package).unwrap();
        }
    }

    let status = child.wait().expect("Failed to wait on child");

    if !status.success() {
        return Err("Failed to install packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages installed".bold());

    println!(
        "{} {}",
        "::".bold().blue(),
        "Updating package index...".bold()
    );

    for new_package in new_packages {
        if !packages.contains(&new_package) {
            file::append_package(get_package_path(), &new_package);
        }
    }

    Ok(())
}
