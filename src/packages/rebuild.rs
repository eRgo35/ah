use colored::Colorize;

use crate::file;
use crate::packages::{
    ask_confirmation, get_package_path, noconfirm_arg, run_command_stdin, PACKAGE_MANAGER,
};

pub fn rebuild(noconfirm: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "::".bold().green(),
        "Upgrading & syncing packages...".bold()
    );

    if !ask_confirmation()? {
        return Err("Operation aborted".into());
    }

    let packages = file::read_packages(get_package_path());

    let packages = packages
        .into_iter()
        .filter(|p| !p.contains("#") && !p.is_empty())
        .collect::<Vec<String>>();

    let input = packages.join("\n");

    let mut args: Vec<&str> = vec!["--color", "always", "-Syu", "--needed"];
    args.extend(noconfirm_arg(noconfirm));
    args.push("-");

    let code = run_command_stdin(PACKAGE_MANAGER, &args, &input)?;

    if code != 0 {
        return Err("Failed to upgrade & sync packages".into());
    }

    println!(
        "{} {}",
        "::".bold().green(),
        "Packages upgraded & synced".bold()
    );
    Ok(())
}
