use colored::Colorize;
use std::path::PathBuf;
use std::io::{self, Write};

pub mod rebuild;
pub mod sync;
pub mod upgrade;
pub mod install;
pub mod remove;
pub mod find;

pub use rebuild::rebuild;
pub use sync::sync;
pub use upgrade::upgrade;
pub use install::install;
pub use remove::remove;
pub use find::find;

fn get_package_path() -> PathBuf {
  let home_dir = std::env::var("HOME").unwrap();
  
  PathBuf::from(home_dir).join("packages")
}

fn ask_confirmation() -> Result<bool, io::Error> {
  print!("{} {}", "::".bold().blue(), "Do you want to continue? [Y/n] ");
  io::stdout().flush()?;

  let mut input = String::new();
  io::stdin().read_line(&mut input)?;

  let input = input.trim().to_lowercase();
  Ok(input.is_empty() || input == "y")
}