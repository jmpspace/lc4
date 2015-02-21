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
  pub struct IMM11 { pub value : i16 }
  #[derive(PartialEq, Eq)]
  pub struct IMM9 { pub value : i16 }
  #[derive(PartialEq, Eq)]
  pub struct IMM7 { pub value : i16 }
  #[derive(PartialEq, Eq)]
  pub struct IMM6 { pub value : i16 }
  #[derive(PartialEq, Eq)]
  pub struct IMM5 { pub value : i16 }

  #[derive(PartialEq, Eq)]
  pub struct UIMM8 { pub value : u16 }
  #[derive(PartialEq, Eq)]
  pub struct UIMM7 { pub value : u16 }
  #[derive(PartialEq, Eq)]
  pub struct UIMM4 { pub value : u16 }

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
  enum DecodeError { Exhaustive, BadOpcode }

  trait Controller {
    fn decode(self) -> Result<Inst, DecodeError>;
  }
    
  macro_rules! opcode {
    ( $inst:expr ) => { ($inst >> 12 ) & 0xF };
  }
  
  macro_rules! br_opcode {
    ( $inst:expr ) => { ($inst >> 9 ) & 0x7 };
  }
  
  macro_rules! arith_opcode {
    ( $inst:expr ) => { ($inst >> 3 ) & 0x7 };
  }
  
  macro_rules! cmp_opcode {
    ( $inst:expr ) => { ($inst >> 7 ) & 0x3 };
  }
  
  macro_rules! rd {
    ( $inst:expr ) => { (($inst >> 9 ) & 0x7) as RName };
  }
  
  macro_rules! rs {
    ( $inst:expr ) => { (($inst >> 6 ) & 0x7) as RName };
  }
  
  macro_rules! rt {
    ( $inst:expr ) => { (($inst >> 0 ) & 0x7) as RName };
  }
  
  macro_rules! imm9 {
    ( $inst:expr ) => { IMM9{value : (($inst as i16) << 7) >> 7} };
  }
  
  macro_rules! imm7 {
    ( $inst:expr ) => { IMM7{value : (($inst as i16) << 9) >> 9} };
  }
  
  macro_rules! imm5 {
    ( $inst:expr ) => { IMM5{value : (($inst as i16) << 11) >> 11} };
  }
  
  macro_rules! uimm7 {
    ( $inst:expr ) => { UIMM7{value : (($inst as u16) << 9) >> 9} };
  }  
  
  impl Controller for u16 {
    fn decode(self) -> Result<Inst, DecodeError> {
      match opcode!(self) {
        
        0b0000 => 
          match br_opcode!(self) {
            0b000 => Ok(Inst::NOP                      ),
            a     => Ok(Inst::BR (a as CC, imm9!(self)))
          },
        
        0b0001 =>
          match arith_opcode!(self) {
            0b000 => Ok(Inst::ADD  (rd!(self), rs!(self),   rt!(self))),
            0b001 => Ok(Inst::MUL  (rd!(self), rs!(self),   rt!(self))),
            0b010 => Ok(Inst::SUB  (rd!(self), rs!(self),   rt!(self))),
            0b011 => Ok(Inst::DIV  (rd!(self), rs!(self),   rt!(self))),
            _     => Ok(Inst::ADDi (rd!(self), rs!(self), imm5!(self)))
          },
        
        0b0010 =>
          match cmp_opcode!(self) {
            0b00 => Ok(Inst::CMP   (rd!(self),    rt!(self))),
            0b01 => Ok(Inst::CMPu  (rd!(self),    rt!(self))),
            0b10 => Ok(Inst::CMPi  (rd!(self),  imm7!(self))),
            0b11 => Ok(Inst::CMPiu (rd!(self), uimm7!(self))),
            _    => Err(DecodeError::Exhaustive)
          },
        
        _ => Err(DecodeError::BadOpcode)
        
      }
    }
  }
  
  #[test]
  fn decode_unit_tests () {
    assert!(0.decode() == Ok(Inst::NOP));
    
    assert!(0b0000011000000110.decode() == Ok(Inst::BR(Z | P, IMM9{value:   6})));
    assert!(0b0000100000010110.decode() == Ok(Inst::BR(N    , IMM9{value:  22})));
    assert!(0b0000101111101110.decode() == Ok(Inst::BR(N | P, IMM9{value: -18})));
    
    assert!(0b0001010001010100.decode() == Ok(Inst::SUB(R2, R1, R4)));
    
    assert!(0b0010011101101001.decode() == Ok(Inst::CMPi(R3, IMM7{value: -23})));
    assert!(0b0010011111101001.decode() == Ok(Inst::CMPiu(R3, UIMM7{value: 105})));
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