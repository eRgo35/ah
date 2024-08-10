use colored::Colorize;
use std::io::{self, Write};
use std::path::PathBuf;

pub mod choose_install;
pub mod find;
pub mod full_upgrade;
pub mod install;
pub mod rebuild;
pub mod remove;
pub mod sync;
pub mod upgrade;

pub use choose_install::choose_install;
pub use find::find;
pub use full_upgrade::full_upgrade;
pub use install::install;
pub use rebuild::rebuild;
pub use remove::remove;
pub use sync::sync;
pub use upgrade::upgrade;

const SYSTEM_UPDATER: &str = "topgrade";
const PACKAGE_MANAGER: &str = "paru";

fn get_package_path() -> PathBuf {
    let home_dir = std::env::var("HOME").unwrap();

    PathBuf::from(home_dir).join("packages")
}

fn ask_confirmation() -> Result<bool, io::Error> {
    print!("{} Do you want to continue? [Y/n] ", "::".bold().blue());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input == "y")
}
