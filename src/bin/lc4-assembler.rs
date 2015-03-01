#![feature(box_syntax, core, env)]

extern crate core;
extern crate lc4;

use std::env::args;
use std::vec::Vec;
use std::str::StrExt;

use lc4::assembler::*;

pub fn main() -> () {
  let ref source_file: String = match args().nth(1) {
    Some(arg) => arg,
    None => {
      println!("Missing source file argument");
      return
    }
  };
  
  let assm_lines: Vec<Assm> = match read_assembly_file(source_file) {
    Err(err) => panic!("{:?}",err),
    Ok(assms) => assms
  };

  // Here could append many assm_lines together

  let assm_data: AssmData = assemble(assm_lines);

  println!("Debug labels:");
  for (l,addr) in assm_data.labels.iter() {
    println!("  Label {:?} = {:?}", l, addr);
  }

  println!("");

  for addr in 0..assm_data.heap {
    println!("{:?} {:?}", addr, assm_data.memory[addr as usize]);
  }

  println!("Heap starts at {:?}", assm_data.heap);

  let out_file = StrExt::replace(source_file.as_slice(), ".lc4", ".lc4obj");

  match write_object_file(assm_data, out_file) {
    Err(err) => panic!("{:?}",err),
    Ok(()) => ()
  }

}
