use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn tempdir() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("ah_test_{}", std::process::id()));
    path
}

#[test]
fn read_packages_filtered_drops_comments_and_blanks() {
    let dir = tempdir();
    fs::create_dir_all(&dir).unwrap();
    let pkg_path = dir.join("packages.txt");

    let mut file = fs::File::create(&pkg_path).unwrap();
    writeln!(file, "pkg1").unwrap();
    writeln!(file, "# this is a comment").unwrap();
    writeln!(file).unwrap();
    writeln!(file, "pkg2").unwrap();
    writeln!(file, "  # indented comment").unwrap();
    writeln!(file, "pkg3 # inline comment").unwrap();
    drop(file);

    let result = ah_pkg::file::read_packages_filtered(pkg_path.clone());
    assert_eq!(result, vec!["pkg1", "pkg2"]);

    fs::remove_dir_all(dir).ok();
}

#[test]
fn write_packages_atomic_writes_content() {
    let dir = tempdir();
    fs::create_dir_all(&dir).unwrap();
    let pkg_path = dir.join("packages_atomic.txt");

    let content = "line1\nline2\nline3\n";
    ah_pkg::file::write_packages_atomic(pkg_path.clone(), content).unwrap();

    let read_back = fs::read_to_string(&pkg_path).unwrap();
    assert_eq!(read_back, content);

    // Verify temp file is gone (rename succeeded)
    let tmp_path = dir.join("packages_atomic.tmp");
    assert!(!tmp_path.exists());

    fs::remove_dir_all(dir).ok();
}
