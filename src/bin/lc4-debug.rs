#![feature(env)]

extern crate lc4;

use std::env::args;

use lc4::assm_data::*;
use lc4::processor::*;

pub fn main() -> () {
  let ref source_file: String = match args().nth(1) {
    Some(arg) => arg,
    None => {
      println!("Missing source file argument");
      return
    }
  };

  let assm_data: AssmData<i16> = match read_object_file(source_file) {
    Err(err) => panic!("{:?}", err),
    Ok(data) => data
  };

  let mut cpu = boot(assm_data);

  println!("Registers {:?} PC {} NZP {}", cpu.regfile, cpu.pc, cpu.nzp);

  for _ in [0;3].iter() {
    match cpu.step() {
      Err(err) => panic!("{:?}", err),
      Ok(()) => ()
    }
    println!("Registers {:?} PC {} NZP {}", cpu.regfile, cpu.pc, cpu.nzp);
  }
  
}