pub mod lc4 {

  // ConditionCode
  pub type CC = u8;

  pub const N : CC = 4;
  pub const Z : CC = 2;
  pub const P : CC = 1;

  pub type RName = u8;

  pub const R0 : RName = 0;
  pub const R1 : RName = 1;
  pub const R2 : RName = 2;
  pub const R3 : RName = 3;
  pub const R4 : RName = 4;
  pub const R5 : RName = 5;
  pub const R6 : RName = 6;
  pub const R7 : RName = 7;

  #[derive(PartialEq, Eq)]
  pub struct IMM11 { pub val : i16 }
  #[derive(PartialEq, Eq)]
  pub struct IMM9 { pub val : i16 }
  #[derive(PartialEq, Eq)]
  pub struct IMM7 { pub val : i8 }
  #[derive(PartialEq, Eq)]
  pub struct IMM6 { pub val : i8 }
  #[derive(PartialEq, Eq)]
  pub struct IMM5 { pub val : i8 }

  #[derive(PartialEq, Eq)]
  pub struct UIMM8 { pub val : u8 }
  #[derive(PartialEq, Eq)]
  pub struct UIMM7 { pub val : u8 }
  #[derive(PartialEq, Eq)]
  pub struct UIMM4 { pub val : u8 }

  // Instructions

  #[derive(PartialEq, Eq)]
  pub enum Inst {
    NOP,
    BR(CC,IMM9),
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
  
  // PSEUDO-instructions

  pub const RET : Inst = Inst::JMPr(R7);
  // LEA(RName, LABEL)
  // LC(RName, LABEL)

  // Assembly Directives
  /*
  .DATA
  .CODE
  .ADDR UIMM16
  .FALIGN
  .FILL IMM16
  .BLKW UIMM16
  .CONST IMM16
  .UCONST UIMM16
  */
  
}

pub mod controller {
  
  use lc4::*;

  #[derive(PartialEq, Eq)]
  enum DecodeError { BadOpcode }

  trait Controller {
    fn decode(&self) -> Result<Inst, DecodeError>;
  }
    
  macro_rules! opcode {
    ( $inst:expr ) => { ($inst & 0xF000 ) >> 12 };
  }
  
  macro_rules! br_opcode {
    ( $inst:expr ) => { ($inst & 0x0E00 ) >> 9 };
  }
  
  macro_rules! decode_imm9 {
    ( $inst:expr ) => { IMM9{val : if $inst & 0x0100 == 0 { $inst & 0x01FF } else { ($inst & 0x01FF) | 0xFE00  } as i16} };
  }
  
  impl Controller for u16 {
    fn decode(&self) -> Result<Inst, DecodeError> {
      match opcode!(self) {
        0b0000 => 
          match br_opcode!(self) {
            0b000 => Ok(Inst::NOP),
            a => Ok(Inst::BR(a as CC, decode_imm9!(self)))
          },
        _ => Err(DecodeError::BadOpcode)
      }
    }
  }
  
  #[test]
  fn decode_unit_tests () {
    assert!((0x0000 as u16).decode() == Ok(Inst::NOP));
  }
}

pub mod cpu {
  
  pub struct CPU {
    pub registers: [u16; 8],
    pub psr: u16,
    pub pc: u16,
    pub nzp: u8,
    pub memory: [u16; 2^16]
  }
  
}