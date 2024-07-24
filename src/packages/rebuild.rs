use colored::Colorize;
use std::process::{Command, Stdio};
use std::io::Write;

use crate::file;
use crate::packages::{ask_confirmation, get_package_path};

const PACKAGE_MANAGER: &str = "paru";

pub fn rebuild(noconfirm: bool) -> Result<(), Box<dyn std::error::Error>> {
  println!("{} {}", "::".bold().green(), "Upgrading & syncing packages...".bold());

  if !ask_confirmation()? {
    return Err("Operation aborted".into());
  }

  let packages = file::read_packages(get_package_path());

  let packages = packages.into_iter()
      .filter(|p| !p.contains("#") && !p.is_empty())
      .collect::<Vec<String>>();

  let noconfirm = if noconfirm { "--noconfirm" } else { "--confirm" };

  let mut child = Command::new(PACKAGE_MANAGER)
      .arg("--color")
      .arg("always")
      .arg("-Syu")
      .arg("--needed")
      .arg(noconfirm)
      .arg("-")
      .stdin(Stdio::piped())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()
      .expect("Failed to execute command");

  if let Some(mut stdin) = child.stdin.take() {
      for package in packages {
        writeln!(stdin, "{}", package).unwrap();
      }
  }

  let status = child.wait().expect("Failed to wait on child");

  if !status.success() {
    return Err("Failed to upgrade & sync packages".into());
  }

  println!("{} {}", "::".bold().green(), "Packages upgraded & synced".bold());
  Ok(())
}
