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
  
  let assms: Vec<Assm> = match read_assembly_file(source_file) {
    Err(err) => panic!("{:?}",err),
    Ok(assms) => assms
  };

  #[derive(Copy, Debug, Eq, PartialEq)]
  enum Section { CODE, DATA };

  let mut section: Section = Section::CODE;

  let mut code_addr: u16 = 0;
  let mut data_addr: u16 = 0;
  
  let mut addr_labels: HashMap<Label, (Section, u16)> = HashMap::new();
  let mut value_labels: HashMap<Label, i16> = HashMap::new();

  /* First-pass, setup labels */

  for &ref assm in assms.iter() {
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
      &Assm::LABEL(ref l) => {
        if addr_labels.contains_key(l) {
          panic!("Cannot have duplicate labels")
        }
        addr_labels.insert(l.clone(), (section, code_addr));
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
        if value_labels.contains_key(l) {
          panic!("Cannot have duplicate labels")
        }
        value_labels.insert(l.clone(), i.value);
      }
      &Assm::LUCONST(ref l, ref u) => {
        if value_labels.contains_key(l) {
          panic!("Cannot have duplicate labels")
        }
        value_labels.insert(l.clone(), u.value as i16);
      }
    }
  }
    
  /* Second pass, write to memory */

  let mut memory = box [Mem::DATA(0); 0x10000];
  let base_data_addr = pad16(code_addr);
  let base_heap_addr = pad16(base_data_addr + data_addr);
  let mut addr: u16 = 0;

  for &ref assm in assms.iter() {
    match assm {
      
      &Assm::LABEL(ref target) => {
        let (label_section, label_addr) = addr_labels[target.clone()];
        match label_section {
          Section::CODE => addr = label_addr,
          Section::DATA => addr = label_addr + base_data_addr
        }
      },

      &Assm::Insn(InsnGen::BR(cc, ref target)) => {
        let (section, label_addr) = addr_labels[target.clone()];
        assert!(section == Section::CODE);
        memory[addr as usize] = Mem::CODE(InsnGen::BR(cc, IMM9{value: (label_addr - (addr + 1)) as i16}));
        addr += 1
      },
      &Assm::Insn(InsnGen::JSR(ref target)) => {
        let (section, label_addr) = addr_labels[target.clone()];
        assert!(section == Section::CODE);
        memory[addr as usize] = Mem::CODE(InsnGen::JSR(IMM11{value: (label_addr - (addr & 0x8000)) as i16 >> 4}));
        addr += 1
      },
      &Assm::Insn(InsnGen::JMP(ref target)) => {
        let (section, label_addr) = addr_labels[target.clone()];
        assert!(section == Section::CODE);
        memory[addr as usize] = Mem::CODE(InsnGen::JMP(IMM11{value: (label_addr - (addr + 1)) as i16}));
        addr += 1
      },
      
      &Assm::Insn(ref insn) => {
        memory[addr as usize] = Mem::CODE(partial_cast(insn));
        addr += 1
      }

      &Assm::RET => {
        memory[addr as usize] = Mem::CODE(InsnGen::JMPr(R7));
        addr += 1
      },

      &Assm::LEA(rd, ref target) => {
        let (section, label_addr) = addr_labels[target.clone()];
        let label_addr = match section {
          Section::CODE => label_addr,
          Section::DATA => label_addr + base_data_addr
        };
        let low = IMM9{value: label_addr as i16 & 0x01FF};
        let high = UIMM8{value: label_addr >> 8};
        memory[addr as usize] = Mem::CODE(InsnGen::CONST(rd, low));
        memory[addr as usize + 1] = Mem::CODE(InsnGen::HICONST(rd, high));
        addr += 2
      },

      &Assm::LC(rd, ref target) => {
        let label_value = value_labels[target.clone()];
        let low = IMM9{value: label_value & 0x01FF};
        let high = UIMM8{value: label_value as u16 >> 8};
        memory[addr as usize] = Mem::CODE(InsnGen::CONST(rd, low));
        memory[addr as usize + 1] = Mem::CODE(InsnGen::HICONST(rd, high));
        addr += 2
      }

      &Assm::CODE => (),
      &Assm::DATA => (),
      &Assm::ADDR(_) => (),
      &Assm::FALIGN => (),

      &Assm::FILL(i) => {
        memory[addr as usize] = Mem::DATA(i.value);
        addr += 1
      },

      &Assm::STRINGZ(_) => panic!("Not implemented"),

      &Assm::BLKW(_) => (),
      &Assm::LCONST(_,_) => (),
      &Assm::LUCONST(_,_) => ()
    }
  }

  println!("Debug labels:");
  for (l,addr) in addr_labels.iter() {
    println!("  Label {:?} = {:?}", l, addr);
  }

  println!("");
  for addr in 0..base_heap_addr {
    println!("{:?} {:?}", addr, memory[addr as usize]);
  }

  println!("Heap starts at {:?}", base_heap_addr);

}
