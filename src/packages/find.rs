use colored::Colorize;
use std::process::Command;

const PACKAGE_MANAGER: &str = "paru";

pub fn find(query: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
  println!("{} {}", "::".bold().green(), "Looking for package...".bold());

  if query.is_empty() {
    return Err("No query provided".into());
  }

  let output = Command::new(PACKAGE_MANAGER)
      .arg("--color")
      .arg("always")
      .arg("-Ss")
      .args(query)
      .output()
      .expect("Failed to execute command");

  print!("{}", String::from_utf8_lossy(&output.stdout));

  Ok(())
}