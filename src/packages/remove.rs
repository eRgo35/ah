use crate::packages::PACKAGE_MANAGER;
use crate::{file, packages::get_package_path};
use colored::Colorize;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn remove(unwanted_packages: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "::".bold().green(), "Removing packages...".bold());

    let packages = file::read_packages(get_package_path());

    let mut packages = packages
        .into_iter()
        .filter(|p| !p.contains("#") && !p.is_empty())
        .collect::<Vec<String>>();

    let mut child = Command::new(PACKAGE_MANAGER)
        .arg("--color")
        .arg("always")
        .arg("-R")
        // .arg("--needed")
        // .arg(noconfirm)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    if let Some(mut stdin) = child.stdin.take() {
        for unwanted_package in unwanted_packages.clone() {
            writeln!(stdin, "{}", unwanted_package).unwrap();
        }
    }

    let status = child.wait().expect("Failed to wait on child");

    if !status.success() {
        return Err("Failed to remove packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages removed".bold());

    println!(
        "{} {}",
        "::".bold().blue(),
        "Updating package index...".bold()
    );

    for unwanted_package in unwanted_packages {
        packages.retain(|p| *p != unwanted_package);
    }

    file::write_packages(get_package_path(), &packages.join("\n"));

    Ok(())
}
