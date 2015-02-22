use architecture::*;

#[derive(PartialEq, Eq)]
pub enum DecodeError { BadOpcode }

pub trait Controller {
  fn decode(self) -> Result<Insn, DecodeError>;
}
  
macro_rules! opcode {
  ( $insn:expr ) => { ($insn >> 12 ) & 0xF };
}

macro_rules! br_opcode {
  ( $insn:expr ) => { ($insn >> 9 ) & 0x7 };
}

macro_rules! arith_opcode {
  ( $insn:expr ) => { ($insn >> 3 ) & 0x7 };
}

macro_rules! cmp_opcode {
  ( $insn:expr ) => { ($insn >> 7 ) & 0x3 };
}

macro_rules! shift_opcode {
  ( $insn:expr ) => { ($insn >> 4 ) & 0x3 };
}  

macro_rules! jump_opcode {
  ( $insn:expr ) => { ($insn >> 11 ) & 0x1 };
}  

macro_rules! rd {
  ( $insn:expr ) => { (($insn >> 9 ) & 0x7) as RName };
}

macro_rules! rs {
  ( $insn:expr ) => { (($insn >> 6 ) & 0x7) as RName };
}

macro_rules! rt {
  ( $insn:expr ) => { (($insn >> 0 ) & 0x7) as RName };
}

macro_rules! imm11 {
  ( $insn:expr ) => { IMM11{value : (($insn as i16) << 5) >> 5} };
}

macro_rules! imm9 {
  ( $insn:expr ) => { IMM9{value : (($insn as i16) << 7) >> 7} };
}

macro_rules! imm7 {
  ( $insn:expr ) => { IMM7{value : (($insn as i16) << 9) >> 9} };
}

macro_rules! imm6 {
  ( $insn:expr ) => { IMM6{value : (($insn as i16) << 10) >> 10} };
}

macro_rules! imm5 {
  ( $insn:expr ) => { IMM5{value : (($insn as i16) << 11) >> 11} };
}

macro_rules! uimm8 {
  ( $insn:expr ) => { UIMM8{value : (($insn as u16) << 8) >> 8} };
}  

macro_rules! uimm7 {
  ( $insn:expr ) => { UIMM7{value : (($insn as u16) << 9) >> 9} };
}    

macro_rules! uimm4 {
  ( $insn:expr ) => { UIMM4{value : (($insn as u16) << 12) >> 12} };
}  

impl Controller for u16 {
  fn decode(self) -> Result<Insn, DecodeError> {
    match opcode!(self) {
      
      // Branching
      0b0000 => 
        match br_opcode!(self) {
          0b000 => Ok(Insn::NOP                      ),
          a     => Ok(Insn::BR (a as CC, imm9!(self)))
        },
      
      // Numeric Arithmetic
      0b0001 =>
        match arith_opcode!(self) {
          0b000 => Ok(Insn::ADD  (rd!(self), rs!(self),   rt!(self))),
          0b001 => Ok(Insn::MUL  (rd!(self), rs!(self),   rt!(self))),
          0b010 => Ok(Insn::SUB  (rd!(self), rs!(self),   rt!(self))),
          0b011 => Ok(Insn::DIV  (rd!(self), rs!(self),   rt!(self))),
          _     => Ok(Insn::ADDi (rd!(self), rs!(self), imm5!(self)))
        },
      
      // Comparison
      0b0010 =>
        match cmp_opcode!(self) {
          0b00 => Ok(Insn::CMP   (rd!(self),    rt!(self))),
          0b01 => Ok(Insn::CMPu  (rd!(self),    rt!(self))),
          0b10 => Ok(Insn::CMPi  (rd!(self),  imm7!(self))),
          _    => Ok(Insn::CMPiu (rd!(self), uimm7!(self))),
        },
      
      // Jump Subroutine
      0b0100 =>
        match jump_opcode!(self) {
          0b0 => Ok(Insn::JSRr (rs!(self))),
          _   => Ok(Insn::JSR  (imm11!(self)))
        },
      
      // Bitwise Arithmetic
      0b0101 =>
        match arith_opcode!(self) {
          0b000 => Ok(Insn::AND  (rd!(self), rs!(self),   rt!(self))),
          0b001 => Ok(Insn::NOT  (rd!(self), rs!(self)             )),
          0b010 => Ok(Insn::OR   (rd!(self), rs!(self),   rt!(self))),
          0b011 => Ok(Insn::XOR  (rd!(self), rs!(self),   rt!(self))),
          _     => Ok(Insn::ANDi (rd!(self), rs!(self), imm5!(self)))
        },
      
      // Memory Access
      0b0110 => Ok(Insn::LDR (rd!(self), rs!(self), imm6!(self))),
      0b0111 => Ok(Insn::STR (rd!(self), rs!(self), imm6!(self))),
      
      // Function Return (C Semantics)
      0b1000 => Ok(Insn::RTI),
      
      // Constant Assignment
      0b1001 => Ok(Insn::CONST (rd!(self), imm9!(self))),
      
      // Shift
      0b1010 => 
        match shift_opcode!(self) {
          0b00 => Ok(Insn::SLL (rd!(self), rs!(self), uimm4!(self))),
          0b01 => Ok(Insn::SRA (rd!(self), rs!(self), uimm4!(self))),
          0b10 => Ok(Insn::SRL (rd!(self), rs!(self), uimm4!(self))),
          _    => Ok(Insn::MOD (rd!(self), rs!(self),    rt!(self)))
        },
      
      // Jump
      0b1100 =>
        match jump_opcode!(self) {
          0b0 => Ok(Insn::JMPr (rs!(self))),
          _   => Ok(Insn::JMP  (imm11!(self)))
        },
      
      // Constant Assignment
      0b1101 => Ok(Insn::HICONST (rd!(self), uimm8!(self))),
      
      // Trap (Jump to privileged subroutine)
      0b1111 => Ok(Insn::TRAP (uimm8!(self))),
      
      _ => Err(DecodeError::BadOpcode)
      
    }
  }
}

#[test]
fn decode_unit_tests () {
  assert!(0.decode() == Ok(Insn::NOP));
  
  assert!(0b0000011000000110.decode() == Ok(Insn::BR(Z | P, IMM9{value:   6})));
  assert!(0b0000100000010110.decode() == Ok(Insn::BR(N    , IMM9{value:  22})));
  assert!(0b0000101111101110.decode() == Ok(Insn::BR(N | P, IMM9{value: -18})));
  
  assert!(0b0001010001010100.decode() == Ok(Insn::SUB(R2, R1, R4)));
  
  assert!(0b0010011101101001.decode() == Ok(Insn::CMPi(R3, IMM7{value: -23})));
  assert!(0b0010011111101001.decode() == Ok(Insn::CMPiu(R3, UIMM7{value: 105})));
  
  assert!(0b0100101001101001.decode() == Ok(Insn::JSR(IMM11{value: 617})));
  assert!(0b0100001001101001.decode() == Ok(Insn::JSRr(R1)));
  
  assert!(0b0101010001010100.decode() == Ok(Insn::OR(R2, R1, R4)));
}