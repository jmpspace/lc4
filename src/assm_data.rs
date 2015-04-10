use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::io;
use std::fs::OpenOptions;
use std::path::Path;

use architecture::*;
use encoder::*;

pub type Label = String;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub fn write_object_file(assm_data: AssmData<Mem>, out_file: &str) -> Result<(), io::Error> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).truncate(true);
    let mut file = try!(options.open(&Path::new(out_file)));
    try!(file.write_u16::<BigEndian>(assm_data.heap));
    for addr in 0..assm_data.heap {
        try!(file.write_i16::<BigEndian>(encode_word(assm_data.memory[addr as usize])))
    }
    Ok(())
}

pub fn read_object_file(in_file: &str) -> Result<AssmData<i16>, io::Error> {
    let mut options = OpenOptions::new();
    options.read(true);
    let mut file = try!(options.open(&Path::new(in_file)));
    let heap: u16 = try!(file.read_u16::<BigEndian>());
    let mut memory: Memory<i16> = box [0;0x10000];
    for addr in 0..heap {
        memory[addr as usize] = try!(file.read_i16::<BigEndian>());
    }
    Ok(AssmData{
        memory: memory,
        labels: HashMap::new(),
        heap: heap
    })
}
