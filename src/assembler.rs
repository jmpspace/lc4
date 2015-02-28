use core::error::FromError;
use architecture::*;
use std::old_io::{BufferedReader, File, IoError};

#[derive(Debug)]
pub enum AssmError { IoError(IoError), ParseError(String) }

impl FromError<IoError> for AssmError {
  fn from_error(err: IoError) -> AssmError {
    AssmError::IoError(err)
  }
}

pub type Label = String;

pub type LInsn = InsnGen<Label, Label>;

#[derive(Debug)]
pub enum Assm {
  LABEL(Label),
  Insn(LInsn),
  RET,
  LEA(RName, Label),
  LC(RName, Label),
  CODE,
  DATA,
  ADDR(UIMM16),
  FALIGN,
  FILL(IMM16),
  STRINGZ(String),
  BLKW(UIMM16),
  LCONST(Label, IMM16),
  LUCONST(Label, UIMM16)
}    

peg_file! lc4_grammar("grammar/lc4.pegjs");

fn read_assembly_line(line: String) -> Result<Assm, AssmError> {
  match lc4_grammar::assm(&line.trim()[..]) {
    Err(err) => Err(AssmError::ParseError(err)),
    Ok(assm) => Ok(assm)
  }
}

pub fn read_assembly_file(filename: &str) -> Result<Vec<Assm>, AssmError> {
  let file = try!(File::open(&Path::new(filename)));
  let mut reader = BufferedReader::new(file);
  let mut assms = Vec::new();
  for line in reader.lines() {
    assms.push(try!(read_assembly_line(try!(line))))
  }
  Ok(assms)
}

#[derive(Clone, Copy, Debug)]
pub enum Mem {
  CODE(Insn),
  DATA(i16)
}