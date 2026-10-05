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
