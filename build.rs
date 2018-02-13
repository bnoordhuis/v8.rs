//use std::env;
use std::process::{Command, exit};

fn main() {
    exit(!Command::new("make")
         .args(&["-f", "makefile.cargo"])
         .status()
         .expect("make error")
         .success() as i32);
}
