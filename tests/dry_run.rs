use ah_pkg::packages::should_confirm;

#[test]
fn dry_run_does_not_require_noconfirm_flag() {
    // Dry-run semantics are an implementation detail of sync/upgrade;
    // a real subprocess test lands in T9. This pins that the helper exists.
    assert!(should_confirm(false));
    assert!(!should_confirm(true));
}
