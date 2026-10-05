use colored::Colorize;

use crate::packages::{ask_confirmation, run_command, SYSTEM_UPDATER};

pub fn full_upgrade(noconfirm: bool, assume_yes: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "::".bold().green(),
        "Initializing full system update...".bold()
    );

    if !ask_confirmation(assume_yes)? {
        return Err("Operation aborted".into());
    }

    let noconfirm_arg = if noconfirm { "-y" } else { "" };

    let code = run_command(SYSTEM_UPDATER, &[noconfirm_arg])?;

    if code != 0 {
        return Err("System upgrade failed".into());
    }

    println!("{} {}", "::".bold().green(), "System upgraded".bold());
    Ok(())
}
