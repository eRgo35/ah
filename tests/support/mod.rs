//! Test support: fake-binary harness for subprocess argv/stdin assertions.
//!
//! Sets `AH_TEST_FAKE_BIN` to point at a temp shell script that records
//! argv (and stdin for `run_command_stdin`) instead of running the real
//! `paru`/`topgrade` binary.

use std::env;
use std::fs;
use std::path::PathBuf;

/// Temp dir holding the fake-bin script.  Kept alive for the lifetime of
/// the support object so the script stays executable.
struct TempFakeBin {
    dir_path: PathBuf,
    script_path: PathBuf,
}

impl TempFakeBin {
    fn new() -> Self {
        let dir_path = env::temp_dir().join("ah-test-fake-bin");
        let _ = fs::create_dir_all(&dir_path);
        let script_path = dir_path.join("fake-bin");

        let script = r#"#!/bin/sh
# record argv: bin + args as TSV
printf '%s\n' "$0" "$@" | paste -sd '\t' - >> "$AH_TEST_RECORD"
# exit with the requested code
exit "${AH_TEST_EXIT:-0}"
"#
        .to_string();

        fs::write(&script_path, script).expect("write fake-bin script");
        // chmod +x
        std::process::Command::new("chmod")
            .args(["+x", &script_path.to_string_lossy()])
            .status()
            .expect("chmod +x fake-bin");

        Self {
            dir_path,
            script_path,
        }
    }
}

impl Drop for TempFakeBin {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir_path);
    }
}

/// Holds the two record file paths so callers can read them back.
pub struct TestHarness {
    _fake_bin: TempFakeBin,
    pub record_path: PathBuf,
    #[allow(dead_code)]
    pub stdin_record_path: PathBuf,
}

impl TestHarness {
    /// Returns the path to the argv record file written by the fake bin.
    #[allow(dead_code)]
    pub fn record_path(&self) -> &PathBuf {
        &self.record_path
    }

    /// Returns the path to the stdin record file (written by `run_command_stdin` override).
    #[allow(dead_code)]
    pub fn stdin_record_path(&self) -> &PathBuf {
        &self.stdin_record_path
    }

    /// Reads the record file and returns one `Vec<String>` per line.
    /// Each line is tab-split into fields.
    pub fn read_record(&self) -> Vec<Vec<String>> {
        let content = fs::read_to_string(&self.record_path).unwrap_or_default();
        content
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split('\t').map(|s| s.to_string()).collect::<Vec<_>>())
            .collect()
    }
}

/// Set up the fake-binary environment and return a harness.
/// Call this at the start of a test.
pub fn setup_env() -> TestHarness {
    let fake_bin = TempFakeBin::new();

    let record_dir = env::temp_dir().join("ah-test-records");
    let _ = fs::create_dir_all(&record_dir);
    let record_path = record_dir.join("record");
    let stdin_record_path = record_dir.join("stdin_record");

    // Truncate/clear the record files
    fs::write(&record_path, "").ok();
    fs::write(&stdin_record_path, "").ok();

    env::set_var("AH_TEST_FAKE_BIN", &fake_bin.script_path);
    env::set_var("AH_TEST_RECORD", &record_path);
    env::set_var("AH_TEST_RECORD_STDIN", &stdin_record_path);
    env::remove_var("AH_TEST_EXIT"); // ensure clean state

    TestHarness {
        _fake_bin: fake_bin,
        record_path,
        stdin_record_path,
    }
}

/// Set `AH_TEST_EXIT` to control the fake binary's exit code.
pub fn set_fake_exit(code: i32) {
    env::set_var("AH_TEST_EXIT", code.to_string());
}

/// Clear all `AH_TEST_*` env vars (call after a test if needed).
#[allow(dead_code)]
pub fn clear_env() {
    env::remove_var("AH_TEST_FAKE_BIN");
    env::remove_var("AH_TEST_RECORD");
    env::remove_var("AH_TEST_RECORD_STDIN");
    env::remove_var("AH_TEST_EXIT");
}
