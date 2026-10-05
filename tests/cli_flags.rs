use ah_pkg::packages::{noconfirm_arg, topgrade_argv};

#[test]
fn test_noconfirm_arg_true() {
    assert_eq!(noconfirm_arg(true), vec!["--noconfirm"]);
}

#[test]
fn test_noconfirm_arg_false() {
    assert_eq!(noconfirm_arg(false), Vec::<&str>::new());
}

#[test]
fn topgrade_argv_omits_flag_when_not_confirming() {
    assert_eq!(topgrade_argv(true), vec!["-y"]);
    assert_eq!(topgrade_argv(false), Vec::<&str>::new());
}

mod support;

use support::{set_fake_exit, setup_env};

/// Verifies that `run_command` records argv when the fake-bin harness is active.
#[test]
fn upgrade_spawns_topgrade_with_correct_argv() {
    let harness = setup_env();
    set_fake_exit(0);

    // Directly exercise run_command with topgrade argv.  full_upgrade would call
    // ask_confirmation first (which reads stdin), so we test run_command directly.
    let _ = ah_pkg::packages::run_command("topgrade", &["-y"]).ok();

    let rows = harness.read_record();
    assert!(
        !rows.is_empty(),
        "expected at least one spawned call, but record was empty"
    );
    let last = rows.last().unwrap();
    assert_eq!(
        last.first().map(|s| s.as_str()),
        Some("topgrade"),
        "expected topgrade as bin"
    );
    assert!(
        last.contains(&"-y".to_string()),
        "expected -y flag in topgrade argv, got: {:?}",
        last
    );
}
