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

pub const SYSTEM_UPDATER: &str = "topgrade";
pub const PACKAGE_MANAGER: &str = "paru";

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

/// Returns the argv slice for `topgrade`. `-y` to skip prompts,
/// or empty when the user didn't pass --yes.
pub fn topgrade_argv(assume_yes: bool) -> Vec<&'static str> {
    if assume_yes {
        vec!["-y"]
    } else {
        Vec::new()
    }
}

/// Returns true if we should actually prompt the user. `assume_yes`
/// (from `--yes`) short-circuits this.
pub fn should_confirm(assume_yes: bool) -> bool {
    !assume_yes
}

pub fn ask_confirmation(assume_yes: bool) -> Result<bool, io::Error> {
    if !should_confirm(assume_yes) {
        return Ok(true);
    }
    print!("{} Do you want to continue? [Y/n] ", "::".bold().blue());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input == "y")
}

pub fn confirm_destructive(action: &str, assume_yes: bool) -> Result<bool, io::Error> {
    if !should_confirm(assume_yes) {
        return Ok(true);
    }
    print!(
        "{} About to {}. Continue? [Y/n] ",
        "::".bold().red(),
        action
    );
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input == "y")
}

/// Run a command and return its exit code (0 = success).
///
/// When `AH_TEST_FAKE_BIN` is set (test harness only), this records `bin` and
/// `args` to the file at `AH_TEST_RECORD` and returns `AH_TEST_EXIT` (default 0)
/// instead of running the real subprocess. This is completely inert in production
/// since `AH_TEST_FAKE_BIN` is never set outside of the test harness.
pub fn run_command(bin: &str, args: &[&str]) -> Result<i32, Box<dyn std::error::Error>> {
    if std::env::var("AH_TEST_FAKE_BIN").is_ok() {
        let record_path = std::env::var("AH_TEST_RECORD").unwrap_or_default();
        let exit_code: i32 = std::env::var("AH_TEST_EXIT")
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .unwrap_or(0);
        // Append argv as a tab-separated line: bin\targ1\targ2...
        if !record_path.is_empty() {
            let argv_line = std::iter::once(bin)
                .chain(args.iter().copied())
                .collect::<Vec<_>>()
                .join("\t");
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&record_path)
                .and_then(|mut f| writeln!(f, "{}", argv_line))
                .ok();
        }
        return Ok(exit_code);
    }

    let status = Command::new(bin).args(args).status()?;
    Ok(status.code().unwrap_or(-1))
}

/// Run a command with stdin input and return its exit code.
///
/// When `AH_TEST_FAKE_BIN` is set (test harness only), this records `bin`, `args`,
/// and `input` to the files at `AH_TEST_RECORD` and `AH_TEST_RECORD_STDIN` respectively,
/// then returns `AH_TEST_EXIT` (default 0). Completely inert in production.
pub fn run_command_stdin(
    bin: &str,
    args: &[&str],
    input: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    if std::env::var("AH_TEST_FAKE_BIN").is_ok() {
        let record_path = std::env::var("AH_TEST_RECORD").unwrap_or_default();
        let stdin_record_path = std::env::var("AH_TEST_RECORD_STDIN").unwrap_or_default();
        let exit_code: i32 = std::env::var("AH_TEST_EXIT")
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .unwrap_or(0);
        // Write argv as tab-separated line
        if !record_path.is_empty() {
            let argv_line = std::iter::once(bin)
                .chain(args.iter().copied())
                .collect::<Vec<_>>()
                .join("\t");
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&record_path)
                .and_then(|mut f| writeln!(f, "{}", argv_line))
                .ok();
        }
        // Write stdin to the stdin record file
        if !stdin_record_path.is_empty() {
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&stdin_record_path)
                .and_then(|mut f| writeln!(f, "{}", input))
                .ok();
        }
        return Ok(exit_code);
    }

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

#[cfg(test)]
mod test_fake_bin {
    use super::*;

    #[test]
    fn test_run_command_records_argv_in_test_mode() {
        use std::env;
        use std::fs;

        let temp = env::temp_dir().join("ah-test-unit");
        let record = temp.join("record");
        fs::create_dir_all(&temp).ok();
        fs::write(&record, "").ok();

        env::set_var("AH_TEST_FAKE_BIN", "1");
        env::set_var("AH_TEST_RECORD", &record);
        env::set_var("AH_TEST_EXIT", "0");

        let result = run_command("echo", &["hello", "world"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        env::remove_var("AH_TEST_FAKE_BIN");
        env::remove_var("AH_TEST_RECORD");
        env::remove_var("AH_TEST_EXIT");

        let content = fs::read_to_string(&record).unwrap();
        assert!(
            !content.is_empty(),
            "expected record to have content, got: {:?}",
            content
        );
        assert!(
            content.contains("echo"),
            "expected 'echo' in record, got: {:?}",
            content
        );
    }
}
