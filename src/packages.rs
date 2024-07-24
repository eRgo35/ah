use colored::Colorize;
use std::process::Command;

const PACKAGE_MANAGER: &str = "paru";

pub fn sync() {
  println!("{} {}", "::".bold().green(), "Syncing packages...".bold());
  todo!();
}

pub fn upgrade() {
  println!("{} {}", "::".bold().green(), "Upgrading packages...".bold());
  todo!();
}

pub fn install(package: &str) {
  println!("{} {}", "::".bold().green(), "Installing packages...".bold());
  todo!();
}

pub fn remove(package: &str) {
  println!("{} {}", "::".bold().green(), "Removing packages...".bold());
  todo!();
}

pub fn find(package: &str) {
  println!("{} {}", "::".bold().green(), "Looking for package...".bold());
  
  let output = Command::new(PACKAGE_MANAGER)
      .arg("--color")
      .arg("always")
      .arg("-Ss")
      .arg(package)
      .output()
      .expect("Failed to execute command");

  print!("{}", String::from_utf8_lossy(&output.stdout));
}