use crate::packages::PACKAGE_MANAGER;
use crate::{file, packages::get_package_path};
use colored::Colorize;

pub fn install(new_packages: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "::".bold().green(),
        "Installing packages...".bold()
    );

    let packages = file::read_packages(get_package_path());

    let packages = packages
        .into_iter()
        .filter(|p| !p.contains("#") && !p.is_empty())
        .collect::<Vec<String>>();

    let input = new_packages.clone().join("\n");
    let args = vec!["--color", "always", "-S", "--needed", "-"];
    let code = crate::packages::run_command_stdin(PACKAGE_MANAGER, &args, &input)?;

    if code != 0 {
        return Err("Failed to install packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages installed".bold());

    println!(
        "{} {}",
        "::".bold().blue(),
        "Updating package index...".bold()
    );

    for new_package in new_packages {
        if !packages.contains(&new_package) {
            file::append_package(get_package_path(), &new_package);
        }
    }

    Ok(())
}
