use colored::Colorize;

use crate::packages::{ask_confirmation, noconfirm_arg, run_command, PACKAGE_MANAGER};

pub fn upgrade(noconfirm: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "::".bold().green(), "Upgrading packages...".bold());

    if !ask_confirmation()? {
        return Err("Operation aborted".into());
    }

    let args: Vec<&str> = vec!["--color", "always", "-Syu"]
        .into_iter()
        .chain(noconfirm_arg(noconfirm).into_iter())
        .collect();

    let code = run_command(PACKAGE_MANAGER, &args)?;

    if code != 0 {
        return Err("Failed to upgrade packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages upgraded".bold());
    Ok(())
}
