use crate::packages::PACKAGE_MANAGER;
use crate::{file, packages::get_package_path};
use colored::Colorize;

pub fn remove(unwanted_packages: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "::".bold().green(), "Removing packages...".bold());

    let mut packages = file::read_packages_filtered(get_package_path());

    let input = unwanted_packages.clone().join("\n");
    let args = vec!["--color", "always", "-R", "-"];
    let code = crate::packages::run_command_stdin(PACKAGE_MANAGER, &args, &input)?;

    if code != 0 {
        return Err("Failed to remove packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages removed".bold());

    println!(
        "{} {}",
        "::".bold().blue(),
        "Updating package index...".bold()
    );

    for unwanted_package in unwanted_packages {
        packages.retain(|p| *p != unwanted_package);
    }

    file::write_packages(get_package_path(), &packages.join("\n"))?;

    Ok(())
}
