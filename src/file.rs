use std::{
  fs::File,
  io::{prelude::*, BufReader}, path::PathBuf,
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

  buf.lines().map(|l| l.expect("Failed to read line")).collect()
}

// pub fn write_packages(path: &str, content: &str) {
//   todo!();
// }