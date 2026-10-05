use colored::Colorize;

use crate::packages::{run_command_output, PACKAGE_MANAGER};

pub fn find(query: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "::".bold().green(),
        "Looking for package...".bold()
    );

    if query.is_empty() {
        return Err("No query provided".into());
    }

    let args: Vec<&str> = vec!["--color", "always", "-Ss"]
        .into_iter()
        .chain(query.iter().map(|s| s.as_str()))
        .collect();

    let output = run_command_output(PACKAGE_MANAGER, &args)?;

    print!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
