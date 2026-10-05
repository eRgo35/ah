use colored::Colorize;

use crate::file;
use crate::packages::{
    ask_confirmation, get_package_path, noconfirm_arg, run_command_stdin, PACKAGE_MANAGER,
};

pub fn sync(noconfirm: bool, assume_yes: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "::".bold().green(), "Syncing packages...".bold());

    if !ask_confirmation(assume_yes)? {
        return Err("Operation aborted".into());
    }

    let packages = file::read_packages_filtered(get_package_path());

    let input = packages.join("\n");

    let mut args: Vec<&str> = vec!["--color", "always", "-S", "--needed"];
    args.extend(noconfirm_arg(noconfirm));
    args.push("-");

    let code = run_command_stdin(PACKAGE_MANAGER, &args, &input)?;

    if code != 0 {
        return Err("Failed to sync packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages synced".bold());
    Ok(())
}
