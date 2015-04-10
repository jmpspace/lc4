#![feature(box_syntax, core)]

extern crate core;
extern crate lc4;

use std::convert::AsRef;
use std::env::args;
use std::vec::Vec;

use lc4::assembler::*;
use lc4::assm_data::*;

pub fn main() -> () {

  let ref source_file: String = match args().nth(1) {
    Some(arg) => arg,
    None => {
      println!("Missing source file argument");
      return
    }
  };
  
  println!("Opening source file {}", source_file);
  let assm_lines: Vec<Assm> = match read_assembly_file(source_file) {
    Err(err) => panic!("{:?}",err),
    Ok(assms) => assms
  };

  println!("Assembling source file {}", source_file);
  let assm_data: AssmData<Mem> = assemble(assm_lines);
  
  println!("Debug labels:");
  for (l,addr) in assm_data.labels.iter() {
    println!("  Label {:?} = {:?}", l, addr);
  }

  println!("");

  for addr in 0..assm_data.heap {
    println!("{:?} {:?}", addr, assm_data.memory[addr as usize]);
  }

  println!("Heap starts at {:?}", assm_data.heap);

  let mut out_file = source_file.clone(); out_file.push_str("obj");

  match write_object_file(assm_data, out_file.as_ref()) {
    Err(err) => panic!("{:?}",err),
    Ok(()) => ()
  }

}
