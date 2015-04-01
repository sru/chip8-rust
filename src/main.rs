extern crate chip8;

use std::io::prelude::*;
use std::fs::File;

use chip8::Chip8;
fn main() {
  let args: Vec<_> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: {} program", args[0]);
    return;
  }

  let mut file = match File::open(&args[1]) {
    Ok(file) => file,
    Err(e) => panic!("Could not open file {}: {}", args[1], e),
  };

  let mut buf = Vec::new();
  match file.read_to_end(&mut buf) {
    Ok(_) => {},
    Err(e) => panic!("Could not read file {}: {}", args[1], e),
  };

  let mut chip8 = Chip8::new();
  chip8.load(&buf);
}
