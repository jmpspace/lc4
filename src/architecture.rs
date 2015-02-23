pub type CC = u8;

pub const N : CC = 4;
pub const Z : CC = 2;
pub const P : CC = 1;

pub type RName = usize;

pub const R0 : RName = 0;
pub const R1 : RName = 1;
pub const R2 : RName = 2;
pub const R3 : RName = 3;
pub const R4 : RName = 4;
pub const R5 : RName = 5;
pub const R6 : RName = 6;
pub const R7 : RName = 7;

#[derive(PartialEq, Eq, Debug)]
pub struct IMM16 { pub value : i16 }
#[derive(PartialEq, Eq, Debug)]
pub struct IMM11 { pub value : i16 }
#[derive(PartialEq, Eq, Debug)]
pub struct IMM9 { pub value : i16 }
#[derive(PartialEq, Eq, Debug)]
pub struct IMM7 { pub value : i16 }
#[derive(PartialEq, Eq, Debug)]
pub struct IMM6 { pub value : i16 }
#[derive(PartialEq, Eq, Debug)]
pub struct IMM5 { pub value : i16 }

#[derive(PartialEq, Eq, Debug)]
pub struct UIMM16 { pub value : u16 }
#[derive(PartialEq, Eq, Debug)]
pub struct UIMM8 { pub value : u16 }
#[derive(PartialEq, Eq, Debug)]
pub struct UIMM7 { pub value : u16 }
#[derive(PartialEq, Eq, Debug)]
pub struct UIMM4 { pub value : u16 }

// Insnructions

#[derive(PartialEq, Eq, Debug)]
pub enum Insn {
  NOP,
  BR(CC, IMM9),
  ADD(RName, RName, RName),
  MUL(RName, RName, RName),
  SUB(RName, RName, RName),
  DIV(RName, RName, RName),
  ADDi(RName, RName, IMM5),
  CMP(RName, RName),
  CMPu(RName, RName),
  CMPi(RName, IMM7),
  CMPiu(RName, UIMM7),
  JSR(IMM11),
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
  JMP(IMM11),
  HICONST(RName, UIMM8),
  TRAP(UIMM8)
}