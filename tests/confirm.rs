use ah_pkg::packages::{confirm_destructive, should_confirm};

#[test]
fn confirm_destructive_symbol_exists() {
    // Real stdin coverage lands in T9 (test harness).
    let _: fn(&str, bool) -> Result<bool, std::io::Error> = confirm_destructive;
}

#[test]
fn should_confirm_logic() {
    assert!(!should_confirm(true));
    assert!(should_confirm(false));
}
