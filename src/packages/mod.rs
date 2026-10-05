use colored::Colorize;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub mod choose_install;
pub mod find;
pub mod full_upgrade;
pub mod install;
pub mod remove;
pub mod sync;
pub mod upgrade;

pub use choose_install::choose_install;
pub use find::find;
pub use full_upgrade::full_upgrade;
pub use install::install;
pub use remove::remove;
pub use sync::sync;
pub use upgrade::upgrade;

const SYSTEM_UPDATER: &str = "topgrade";
const PACKAGE_MANAGER: &str = "paru";

fn get_package_path() -> PathBuf {
    let home_dir = std::env::var("HOME").unwrap();

    PathBuf::from(home_dir).join("packages")
}

pub fn noconfirm_arg(noconfirm: bool) -> Vec<&'static str> {
    if noconfirm {
        vec!["--noconfirm"]
    } else {
        Vec::new()
    }
}

fn ask_confirmation() -> Result<bool, io::Error> {
    print!("{} Do you want to continue? [Y/n] ", "::".bold().blue());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input == "y")
}

/// Run a command and return its exit code (0 = success).
pub fn run_command(bin: &str, args: &[&str]) -> Result<i32, Box<dyn std::error::Error>> {
    let status = Command::new(bin).args(args).status()?;
    Ok(status.code().unwrap_or(-1))
}

/// Run a command with stdin input and return its exit code.
pub fn run_command_stdin(
    bin: &str,
    args: &[&str],
    input: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut child = Command::new(bin)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())?;
    }

    let status = child.wait()?;
    Ok(status.code().unwrap_or(-1))
}

/// Run a command and capture its output.
pub fn run_command_output(
    bin: &str,
    args: &[&str],
) -> Result<std::process::Output, Box<dyn std::error::Error>> {
    Ok(Command::new(bin).args(args).output()?)
}
