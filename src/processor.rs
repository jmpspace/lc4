use architecture::*;
use controller::*;
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
        self.nzp = from_ordering((self.regfile[rd] as u16)
                    .cmp(&(self.regfile[rt] as u16))),
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