use std::collections::HashMap;
use std::old_io::{File, FileAccess, FileMode, IoResult};

use architecture::*;
use encoder::*;

pub type Label = String;

#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Section { CODE, DATA }

#[derive(Clone, Copy, Debug)]
pub enum Mem {
  CODE(Insn),
  DATA(i16)
}

pub type Memory<M> = Box<[M;0x10000]>;

pub struct AssmData<M> {
  pub memory: Memory<M>,
  pub labels: HashMap<Label, (Section, u16)>,
  pub heap: u16
}

pub fn encode_word(mem: Mem) -> i16 {
  match mem {
    Mem::CODE(insn) => encode_insn(insn),
    Mem::DATA(i) => i
  }
}

pub fn write_object_file(assm_data: AssmData<Mem>, out_file: &str) -> IoResult<()> {
  let mut file = try!(File::open_mode(&Path::new(out_file), FileMode::Truncate, FileAccess::Write));
  try!(file.write_be_u16(assm_data.heap));
  for addr in 0..assm_data.heap {
    try!(file.write_be_i16(encode_word(assm_data.memory[addr as usize])))
  }
  Ok(())
}

pub fn read_object_file(in_file: &str) -> IoResult<AssmData<i16>> {
  let mut file = try!(File::open(&Path::new(in_file)));
  let heap: u16 = try!(file.read_be_u16());
  let mut memory: Memory<i16> = box [0;0x10000];
  for addr in 0..heap {
    memory[addr as usize] = try!(file.read_be_i16());
  }
  Ok(AssmData{
    memory: memory,
    labels: HashMap::new(),
    heap: heap
  })
}