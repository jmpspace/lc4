#![feature(box_syntax, core, env)]

extern crate core;
extern crate lc4;

use std::collections::HashMap;
use std::env::args;

use lc4::architecture::*;
use lc4::assembler::*;

pub fn pad16(addr: u16) -> u16 {
  let mut padded = addr & 0xFFF0;
  if padded < addr { padded += 0x10; }
  padded
}

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
    Err(err) => panic!("{:?}",err),
    Ok(assms) => assms
  };

  #[derive(Eq, PartialEq)]
  enum Section { CODE, DATA };

  let mut section: Section = Section::CODE;

  let mut code_addr: u16 = 0;
  let mut code_addr_labels: HashMap<Label, u16> = HashMap::new();

  let mut data_addr: u16 = 0;
  let mut data_addr_labels: HashMap<Label, u16> = HashMap::new();

  let mut data_value_labels: HashMap<Label, i16> = HashMap::new();

  /* First-pass, setup labels */

  for &ref assm in assms.iter() {
    println!("{:?}", assm);
    match assm {

      // Instructions and Pseudo-Instructions
      &Assm::Insn(_) => {
        assert!(section == Section::CODE);
        code_addr += 1
      },
      &Assm::RET => {
        assert!(section == Section::CODE);
        code_addr += 1
      },
      &Assm::LEA(_, _) => {
        assert!(section == Section::CODE);
        code_addr += 1
      },
      &Assm::LC(_, _) => {
        assert!(section == Section::CODE);
        code_addr += 1
      },

      // Assembler Directives
      &Assm::LABEL(ref l) => 
        match section {
          Section::CODE => {
            if code_addr_labels.contains_key(l) {
              panic!("Cannot have duplicate labels")
            }
            code_addr_labels.insert(l.clone(), code_addr);
          },
          Section::DATA => {
            if data_addr_labels.contains_key(l) {
              panic!("Cannot have duplicate labels")
            }
            data_addr_labels.insert(l.clone(), data_addr);
          }
        },

      &Assm::CODE => section = Section::CODE,
      &Assm::DATA => section = Section::DATA,

      &Assm::ADDR(ref u) => 
        match section {
          Section::CODE => code_addr = u.value,
          Section::DATA => data_addr = u.value
        },

      &Assm::FALIGN => {
        match section {
          Section::CODE => code_addr = pad16(code_addr),
          Section::DATA => data_addr = pad16(data_addr)
        }
      },

      &Assm::FILL(_) => {
        assert!(section == Section::DATA);
        data_addr += 1
      },
      &Assm::STRINGZ(ref s) => {
        assert!(section == Section::DATA);
        data_addr += s.len() as u16
      },

      &Assm::BLKW(ref u) => {
        match section {
          Section::CODE => code_addr += u.value,
          Section::DATA => data_addr += u.value
        }
      },

      &Assm::LCONST(ref l, ref i) => {
        if data_value_labels.contains_key(l) {
          panic!("Cannot have duplicate labels")
        }
        data_value_labels.insert(l.clone(), i.value);
      }
      &Assm::LUCONST(ref l, ref u) => {
        if data_value_labels.contains_key(l) {
          panic!("Cannot have duplicate labels")
        }
        data_value_labels.insert(l.clone(), u.value as i16);
      }
    }
  }

  // Reset section addresses
  data_addr = pad16(code_addr);
  code_addr = 0;

  /* Second pass, write to memory */

  let mut memory = box [Mem::DATA(0); 0x10000];
  //let mut memory: [Mem] = repeat(Mem::DATA(0)).take(0x10000).collect();

  for &ref assm in assms.iter() {
    match assm {
      &Assm::Insn(InsnGen::BR(cc, ref target)) => {
        memory[code_addr as usize] = Mem::CODE(InsnGen::BR(cc, IMM9{value: (code_addr_labels[target.clone()] - (code_addr + 1)) as i16}));
        code_addr += 1
      },
      &Assm::Insn(InsnGen::JSR(ref target)) => {
        memory[code_addr as usize] = Mem::CODE(InsnGen::JSR(IMM11{value: (code_addr_labels[target.clone()] - (code_addr & 0x8000)) as i16 >> 4}));
        code_addr += 1
      },
      &Assm::Insn(InsnGen::JMP(ref target)) => {
        memory[code_addr as usize] = Mem::CODE(InsnGen::JMP(IMM11{value: (code_addr_labels[target.clone()] - (code_addr + 1)) as i16}));
        code_addr += 1
      },
      &Assm::Insn(ref insn) => {
        memory[code_addr as usize] = Mem::CODE(partial_cast(insn));
        code_addr += 1
      }

      _ => panic!("Not implemented {:?}", assm)
    }
  }

}