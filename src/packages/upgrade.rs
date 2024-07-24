use colored::Colorize;
use std::process::Command;

use crate::packages::ask_confirmation;

const PACKAGE_MANAGER: &str = "paru";

pub fn upgrade(noconfirm: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "::".bold().green(), "Upgrading packages...".bold());

    if !ask_confirmation()? {
        return Err("Operation aborted".into());
    }

    let noconfirm = if noconfirm {
        "--noconfirm"
    } else {
        "--confirm"
    };

    let mut child = Command::new(PACKAGE_MANAGER)
        .arg("--color")
        .arg("always")
        .arg("-Syu")
        .arg(noconfirm)
        .spawn()
        .expect("Failed to execute command");

    let status = child.wait().expect("Failed to wait on child");

    if !status.success() {
        return Err("Failed to upgrade packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages upgraded".bold());
    Ok(())
}
