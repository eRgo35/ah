use colored::Colorize;

use crate::packages::{ask_confirmation, run_command, topgrade_argv, SYSTEM_UPDATER};

pub fn full_upgrade(assume_yes: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "::".bold().green(),
        "Initializing full system update...".bold()
    );

    if !ask_confirmation(assume_yes)? {
        return Err("Operation aborted".into());
    }

    let argv = topgrade_argv(assume_yes);
    let code = run_command(SYSTEM_UPDATER, &argv)?;
    if code != 0 {
        return Err(format!("topgrade exited with {}", code).into());
    }

    println!("{} {}", "::".bold().green(), "System upgraded".bold());
    Ok(())
}
