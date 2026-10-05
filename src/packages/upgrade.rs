use colored::Colorize;
use std::process::Command;

use crate::packages::{ask_confirmation, noconfirm_arg, PACKAGE_MANAGER};

pub fn upgrade(noconfirm: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "::".bold().green(), "Upgrading packages...".bold());

    if !ask_confirmation()? {
        return Err("Operation aborted".into());
    }

    let mut child = Command::new(PACKAGE_MANAGER)
        .arg("--color")
        .arg("always")
        .arg("-Syu")
        .args(noconfirm_arg(noconfirm))
        .spawn()
        .expect("Failed to execute command");

    let status = child.wait().expect("Failed to wait on child");

    if !status.success() {
        return Err("Failed to upgrade packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages upgraded".bold());
    Ok(())
}
