extern crate emoji;
extern crate sha256;

use sha256::digest_file;
use std::path::Path;

fn main() {
  let input = Path::new("./file.txt");
  let val   = digest_file(input).unwrap();
  println!("{}", val);
}

