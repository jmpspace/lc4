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

#[derive(Debug)]
pub enum Assm { 
  NOP,
  BR(CC, Label),
  ADD(RName, RName, RName),
  MUL(RName, RName, RName),
  SUB(RName, RName, RName),
  DIV(RName, RName, RName),
  ADDi(RName, RName, IMM5),
  CMP(RName, RName),
  CMPu(RName, RName),
  CMPi(RName, IMM7),
  CMPiu(RName, UIMM7),
  JSR(Label),
  JSRr(RName),
  AND(RName, RName, RName),
  NOT(RName, RName),
  OR(RName, RName, RName),
  XOR(RName, RName, RName),
  ANDi(RName, RName, IMM5),
  LDR(RName, RName, IMM6),
  STR(RName, RName, IMM6),
  RTI,
  CONST(RName, IMM9),
  SLL(RName, RName, UIMM4),
  SRA(RName, RName, UIMM4),
  SRL(RName, RName, UIMM4),
  MOD(RName, RName, RName),
  JMPr(RName),
  JMP(Label),
  HICONST(RName, UIMM8),
  TRAP(UIMM8),
  RET,
  LEA(RName, Label),
  LC(RName, Label),
  DATA,
  CODE,
  ADDR(UIMM16),
  FALIGN,
  FILL(IMM16),
  STRINGZ(String),
  BLKW(UIMM16),
  LCONST(Label, IMM16),
  LUCONST(Label, UIMM16)
}    

peg_file! lc4_grammar("lc4_assembly.rustpeg");

pub struct AssemblyFile {
  reader: BufferedReader<File>
}

pub fn open_assembly_file(filename: &str) -> Result<AssemblyFile, IoError> {
  let file = try!(File::open(&Path::new(filename)));
  let reader = BufferedReader::new(file);
  Ok(AssemblyFile{reader: reader})
}

impl Iterator for AssemblyFile {
  type Item = Result<Assm, AssmError>;
  fn next(&mut self) -> Option<Result<Assm, AssmError>> {
    match self.reader.lines().next() {
      None => None,
      Some(Err(err)) => Some(Err(AssmError::IoError(err))),
      Some(Ok(line)) => {
        match lc4_grammar::assm(&line.trim()[..]) {
          Err(err) => Some(Err(AssmError::ParseError(err))),
          Ok(assm) => Some(Ok(assm))
        }
      }
    }
  }
}
