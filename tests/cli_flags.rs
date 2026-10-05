use ah_pkg::packages::noconfirm_arg;

#[test]
fn test_noconfirm_arg_true() {
    assert_eq!(noconfirm_arg(true), vec!["--noconfirm"]);
}

#[test]
fn test_noconfirm_arg_false() {
    assert_eq!(noconfirm_arg(false), Vec::<&str>::new());
}
