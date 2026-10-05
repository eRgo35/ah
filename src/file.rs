use std::{
    fs::{self, File, OpenOptions},
    io::{prelude::*, BufReader, Write},
    path::PathBuf,
};

pub fn read_packages(path: PathBuf) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Failed to read line"))
        .collect()
}

pub fn read_packages_filtered(path: PathBuf) -> Vec<String> {
    read_packages(path)
        .into_iter()
        .filter(|p| !p.is_empty() && !p.contains('#'))
        .collect()
}

pub fn append_package(path: PathBuf, package: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;

    writeln!(file, "{}", package)
}

pub fn write_packages(path: PathBuf, content: &str) -> std::io::Result<()> {
    write_packages_atomic(path, content)
}

pub fn write_packages_atomic(path: PathBuf, content: &str) -> std::io::Result<()> {
    let tmp_path = path.with_extension("tmp");
    let mut file = File::create(&tmp_path)?;
    file.write_all(content.as_bytes())?;
    fs::rename(tmp_path, path)?;
    Ok(())
}
