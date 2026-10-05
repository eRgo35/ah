use ah_pkg::packages::confirm_destructive;

#[test]
fn confirm_destructive_symbol_exists() {
    // Real stdin coverage lands in T9 (test harness).
    let _: fn(&str) -> Result<bool, std::io::Error> = confirm_destructive;
}
