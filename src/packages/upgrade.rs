use colored::Colorize;

use crate::packages::{ask_confirmation, noconfirm_arg, run_command, PACKAGE_MANAGER};

pub fn upgrade(
    noconfirm: bool,
    dry_run: bool,
    assume_yes: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "::".bold().green(), "Upgrading packages...".bold());

    if dry_run {
        println!(
            "Would upgrade all installed packages via: paru -Syu{}",
            if noconfirm { " --noconfirm" } else { "" }
        );
        return Ok(());
    }

    if !ask_confirmation(assume_yes)? {
        return Err("Operation aborted".into());
    }

    let args: Vec<&str> = vec!["--color", "always", "-Syu"]
        .into_iter()
        .chain(noconfirm_arg(noconfirm))
        .collect();

    let code = run_command(PACKAGE_MANAGER, &args)?;

    if code != 0 {
        return Err("Failed to upgrade packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages upgraded".bold());
    Ok(())
}
