use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, BufReader},
    path::PathBuf,
};

// pub fn read_config(path: &str) -> Vec<String> {
//   todo!();
// }

// pub fn write_config(path: &str, content: &str) {
//   todo!();
// }

pub fn read_packages(path: PathBuf) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Failed to read line"))
        .collect()
}

pub fn append_package(path: PathBuf, package: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    if let Err(err) = writeln!(file, "{}", package) {
        eprintln!("Couldn't write to file: {}", err);
    }
}

pub fn write_packages(path: PathBuf, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open file");

    if let Err(err) = writeln!(file, "{}", content) {
        eprintln!("Couldn't write to file: {}", err);
    }
}
