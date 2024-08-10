use colored::Colorize;
use std::process::Command;

use crate::packages::{ask_confirmation, SYSTEM_UPDATER};

pub fn full_upgrade(noconfirm: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "::".bold().green(),
        "Initializing full system update...".bold()
    );

    if !ask_confirmation()? {
        return Err("Operation aborted".into());
    }

    let noconfirm = if noconfirm { "-y" } else { "" };

    let mut child = Command::new(SYSTEM_UPDATER)
        .arg(noconfirm)
        .spawn()
        .expect("Failed to execute command");

    let status = child.wait().expect("Failed to wait on child");

    if !status.success() {
        return Err("System upgrade failed".into());
    }

    println!("{} {}", "::".bold().green(), "System upgraded".bold());
    Ok(())
}
