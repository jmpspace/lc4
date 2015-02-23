#![feature(core, env)]

extern crate core;
extern crate lc4;

use std::collections::HashMap;
use std::env::args;

use lc4::assembler::*;

pub fn main() -> () {
  let ref source_file = match args().nth(1) {
    Some(arg) => arg,
    None => {
      println!("Missing source file argument");
      return
    }
  };

  println!("Using source file '{}'", source_file);

  let assms: Vec<Assm> = match read_assembly_file(source_file) {
    Err(err) => panic!(err),
    Ok(assms) => assms
  };

  let mut curr_addr: u16 = 0;
  let mut lookup: HashMap<Label, u16> = HashMap::new();

  for &ref assm in assms.iter() {
    match assm {
      // Regular instructions and RET
      &Assm::LEA(_, _) => { curr_addr += 2 },
      &Assm::LC(_, _) => { curr_addr += 2 },
      &Assm::LABEL(ref l) => { 
        if lookup.contains_key(l) {
          panic!("Cannot have duplicate labels")
        }
        lookup.insert(l.clone(), curr_addr);
      },      
      &Assm::CODE => panic!("Not implemented - set code region"),
      &Assm::DATA => panic!("Not implemented - set data region"),
      &Assm::ADDR(ref u) => curr_addr = u.value,
      &Assm::FALIGN => panic!("Not implemented - pad address to FFF0"),
      &Assm::STRINGZ(ref s) => {
        curr_addr += s.len() as u16
      },
      &Assm::BLKW(ref u) => {
        curr_addr += u.value
      },
      &Assm::LCONST(ref l, ref i) => {
        if lookup.contains_key(l) {
          panic!("Cannot have duplicate labels")
        }
        lookup.insert(l.clone(), i.value as u16);
      }
      &Assm::LUCONST(ref l, ref u) => {
        if lookup.contains_key(l) {
          panic!("Cannot have duplicate labels")
        }
        lookup.insert(l.clone(), u.value);
      },
      _ => { curr_addr += 1 }
    }
  }

  //println!("{:?}", assms)
}