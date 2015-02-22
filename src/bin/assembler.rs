#![feature(core, env)]

extern crate core;
extern crate lc4;

use lc4::assembler::*;
use std::env::args;

pub fn main() -> () {
  let ref source_file = match args().nth(1) {
    Some(arg) => arg,
    None => {
      println!("Missing source file argument");
      return
    }
  };

  println!("Using source file '{}'", source_file);

  let assms = read_assembly_file(source_file);

  println!("{:?}", assms);
}