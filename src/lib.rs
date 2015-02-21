
#![feature(core)]
#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate core;

pub mod lc4 {

  pub mod architecture {

    // ConditionCode
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

    #[derive(PartialEq, Eq)]
    pub struct IMM16 { pub value : i16 }
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
    pub struct UIMM16 { pub value : i16 }
    #[derive(PartialEq, Eq)]
    pub struct UIMM8 { pub value : u16 }
    #[derive(PartialEq, Eq)]
    pub struct UIMM7 { pub value : u16 }
    #[derive(PartialEq, Eq)]
    pub struct UIMM4 { pub value : u16 }

    // Insnructions

    #[derive(PartialEq, Eq)]
    pub enum Insn {
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
        
  }
  
  pub mod assembler {
    
    use lc4::architecture::*;
    
    type Label = String;
    
    enum Assembly { 
      Insn(Insn),
      RET,
      LEA(RName, Label),
      LC(RName, Label),
      DATA,
      CODE,
      ADDR(UIMM16),
      FALIGN,
      FILL(IMM16),
      BLK(UIMM16),
      CONST(Label, IMM16),
      UCONST(Label, UIMM16)
    }    
    
    peg_file! modname("lc4_assembly.rustpeg");
    
  }
  
  pub mod controller {
    
    use lc4::architecture::*;

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
  }

  pub mod processor {
    
    use lc4::architecture::*;
    use lc4::controller::*;
    use core::error::FromError;
    use std::cmp::Ordering;
    
    pub struct CPU {
      pub regfile: [i16; 8],
      pub priv_status: bool,
      pub pc: u16,
      pub nzp: CC,
      pub memory: [i16; 2^16]
    }
    
    trait Simulate {
      fn execute(&mut self, insn: Insn) -> Result<(), CPUError>;
      fn step(&mut self) -> Result<(), CPUError>;
    }
    
    enum CPUError { DecodeError(DecodeError), Unauthorized }
    
    impl FromError<DecodeError> for CPUError {
      fn from_error(err: DecodeError) -> CPUError {
        CPUError::DecodeError(err)
      }
    }
    
    fn from_ordering(ord: Ordering) -> CC {
      match ord {
        Ordering::Less => N,
        Ordering::Equal => Z,
        Ordering::Greater => P
      }
    }
    
    impl Simulate for CPU {
      
      fn execute(&mut self, insn: Insn) -> Result<(), CPUError> {
        let mut pc_incr = true;
        match insn {
          Insn::NOP => {},
          
          Insn::BR(cc, offset) => 
            if cc & self.nzp != 0 { 
              self.pc += offset.value as u16 
            },
          
          Insn::ADD(rd, rs, rt) => 
            self.regfile[rd] = self.regfile[rs] + self.regfile[rt],
          Insn::MUL(rd, rs, rt) => 
            self.regfile[rd] = self.regfile[rs] * self.regfile[rt],
          Insn::SUB(rd, rs, rt) => 
            self.regfile[rd] = self.regfile[rs] - self.regfile[rt],
          Insn::DIV(rd, rs, rt) => 
            self.regfile[rd] = self.regfile[rs] / self.regfile[rt],
          Insn::ADDi(rd, rs, n) => 
            self.regfile[rd] = self.regfile[rs] + n.value,
          
          Insn::CMP(rd, rt) => 
            self.nzp = from_ordering(self.regfile[rd].cmp(&self.regfile[rt])),
          Insn::CMPu(rd, rt) => 
            self.nzp = from_ordering((self.regfile[rd] as u16).cmp(&(self.regfile[rt] as u16))),
          Insn::CMPi(rd, test) => 
            self.nzp = from_ordering(self.regfile[rd].cmp(&test.value)),
          Insn::CMPiu(rd, test) => 
            self.nzp = from_ordering((self.regfile[rd] as u16).cmp(&test.value)),
          
          Insn::JSR(target) => { 
            pc_incr = false; 
            self.regfile[R7] = self.pc as i16 + 1; 
            self.pc = (self.pc & 0x8000) | (target.value << 4) as u16 
          }
          Insn::JSRr(rs) => { 
            pc_incr = false; 
            self.regfile[R7] = self.pc as i16 + 1; 
            self.pc = self.regfile[rs] as u16
          }
          
          Insn::AND(rd, rs, rt) => 
            self.regfile[rd] = self.regfile[rs] & self.regfile[rt],
          Insn::NOT(rd, rs)     => 
            self.regfile[rd] = !self.regfile[rs],
          Insn::OR (rd, rs, rt) => 
            self.regfile[rd] = self.regfile[rs] | self.regfile[rt],
          Insn::XOR(rd, rs, rt) => 
            self.regfile[rd] = self.regfile[rs] ^ self.regfile[rt],
          Insn::ANDi(rd, rs, n) => 
            self.regfile[rd] = self.regfile[rs] & (n.value as i16),
          
          Insn::LDR(rd, rs, offset) => {
            let addr = (self.regfile[rs] as i16 + offset.value) as usize;
            if !self.priv_status && addr >= 0x8000 { 
              return Err(CPUError::Unauthorized) 
            };
            self.regfile[rd] = self.memory[addr]
          },
          Insn::STR(rd, rs, offset) =>  {
            let addr = (self.regfile[rs] as i16 + offset.value) as usize;
            if !self.priv_status && addr >= 0x8000 { 
              return Err(CPUError::Unauthorized) 
            };
            self.memory[addr] = self.regfile[rd]
          },
          
          Insn::RTI => { 
            pc_incr = false; 
            self.pc = self.regfile[R7] as u16; 
            self.priv_status = false 
          },
          
          Insn::CONST(rd, c) => self.regfile[rd] = c.value,
          
          Insn::SLL(rd, rs, amount) => 
            self.regfile[rd] = self.regfile[rs] << amount.value,
          Insn::SRA(rd, rs, amount) => 
            self.regfile[rd] = ((self.regfile[rs] as u16) >> amount.value) as i16,
          Insn::SRL(rd, rs, amount) => 
            self.regfile[rd] = self.regfile[rs] >> amount.value,
          Insn::MOD(rd, rs, rt) => 
            self.regfile[rd] = self.regfile[rs] % self.regfile[rt],
          
          Insn::JMPr(rs) => {
            pc_incr = false;
            self.pc = self.regfile[rs] as u16
          },
          Insn::JMP(target) => self.pc = ((self.pc as i16) + target.value) as u16,
          
          Insn::HICONST(rd, c) => 
            self.regfile[rd] = (self.regfile[rd] & 0xFF) | ((c.value << 8) as i16),
          
          Insn::TRAP(target) => {
            pc_incr = false;
            self.pc = 0x8000 | target.value;
            self.priv_status = true;
          }
        };
        
        if pc_incr { self.pc += 1 };
        
        Ok(())
      }
      
      fn step(&mut self) -> Result<(), CPUError> {
        let raw_insn = self.memory[self.pc as usize];
        let insn = try!((raw_insn as u16).decode());
        self.execute(insn)
      }
      
    }
    
  }
}