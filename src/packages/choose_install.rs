use colored::Colorize;

use crate::packages::{run_command_stdin, PACKAGE_MANAGER};

pub fn choose_install(query: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "::".bold().green(),
        "Looking for package...".bold()
    );

    if query.is_empty() {
        return Err("No query provided".into());
    }

    let input = query.join("\n");
    let args = vec!["--color", "always", "s", "-"];
    let code = run_command_stdin(PACKAGE_MANAGER, &args, &input)?;

    if code != 0 {
        return Err("Failed to install packages".into());
    }

    println!("{} {}", "::".bold().green(), "Packages installed".bold());

    println!(
        "{} {}",
        "::".bold().red(),
        "Package index has not been updated!".bold()
    );

    Ok(())
}
