use architecture::*;
use assm_data::*;
use controller::*;
use std::convert::From;
use std::cmp::Ordering;

pub struct CPU {
  pub regfile: [i16; 8],
  pub priv_status: bool,
  pub pc: u16,
  pub nzp: CC,
  pub memory: Memory<i16>
}

pub trait Simulate {
  fn execute(&mut self, insn: Insn) -> Result<(), CPUError>;
  fn step(&mut self) -> Result<(), CPUError>;
}

#[derive(Debug)]
pub enum CPUError { DecodeError(DecodeError), Unauthorized }

impl From<DecodeError> for CPUError {
  fn from(err: DecodeError) -> CPUError {
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

pub fn boot(assm_data: AssmData<i16>) -> CPU {
  CPU{
    regfile: [0;8],
    priv_status: false,
    pc: 0,
    nzp: Z,
    memory: assm_data.memory
  }
}

impl Simulate for CPU {
  
  fn execute(&mut self, insn: Insn) -> Result<(), CPUError> {
    let mut pc_incr = true;
    match insn {
      InsnGen::NOP => {},
      
      InsnGen::BR(cc, offset) => 
        if cc & self.nzp != 0 { 
          self.pc += offset.value as u16 
        },
      
      InsnGen::ADD(rd, rs, rt) => 
        self.regfile[rd] = self.regfile[rs] + self.regfile[rt],
      InsnGen::MUL(rd, rs, rt) => 
        self.regfile[rd] = self.regfile[rs] * self.regfile[rt],
      InsnGen::SUB(rd, rs, rt) => 
        self.regfile[rd] = self.regfile[rs] - self.regfile[rt],
      InsnGen::DIV(rd, rs, rt) => 
        self.regfile[rd] = self.regfile[rs] / self.regfile[rt],
      InsnGen::ADDi(rd, rs, n) => 
        self.regfile[rd] = self.regfile[rs] + n.value,
      
      InsnGen::CMP(rd, rt) => 
        self.nzp = from_ordering(self.regfile[rd].cmp(&self.regfile[rt])),
      InsnGen::CMPu(rd, rt) => 
        self.nzp = from_ordering((self.regfile[rd] as u16)
                    .cmp(&(self.regfile[rt] as u16))),
      InsnGen::CMPi(rd, test) => 
        self.nzp = from_ordering(self.regfile[rd].cmp(&test.value)),
      InsnGen::CMPiu(rd, test) => 
        self.nzp = from_ordering((self.regfile[rd] as u16).cmp(&test.value)),
      
      InsnGen::JSR(target) => { 
        pc_incr = false; 
        self.regfile[R7] = self.pc as i16 + 1; 
        self.pc = (self.pc & 0x8000) | (target.value << 4) as u16 
      }
      InsnGen::JSRr(rs) => { 
        pc_incr = false; 
        self.regfile[R7] = self.pc as i16 + 1; 
        self.pc = self.regfile[rs] as u16
      }
      
      InsnGen::AND(rd, rs, rt) => 
        self.regfile[rd] = self.regfile[rs] & self.regfile[rt],
      InsnGen::NOT(rd, rs)     => 
        self.regfile[rd] = !self.regfile[rs],
      InsnGen::OR (rd, rs, rt) => 
        self.regfile[rd] = self.regfile[rs] | self.regfile[rt],
      InsnGen::XOR(rd, rs, rt) => 
        self.regfile[rd] = self.regfile[rs] ^ self.regfile[rt],
      InsnGen::ANDi(rd, rs, n) => 
        self.regfile[rd] = self.regfile[rs] & (n.value as i16),
      
      InsnGen::LDR(rd, rs, offset) => {
        let addr = (self.regfile[rs] as i16 + offset.value) as usize;
        if !self.priv_status && addr >= 0x8000 { 
          return Err(CPUError::Unauthorized) 
        };
        self.regfile[rd] = self.memory[addr]
      },
      InsnGen::STR(rd, rs, offset) =>  {
        let addr = (self.regfile[rs] as i16 + offset.value) as usize;
        if !self.priv_status && addr >= 0x8000 { 
          return Err(CPUError::Unauthorized) 
        };
        self.memory[addr] = self.regfile[rd]
      },
      
      InsnGen::RTI => { 
        pc_incr = false; 
        self.pc = self.regfile[R7] as u16; 
        self.priv_status = false 
      },
      
      InsnGen::CONST(rd, c) => self.regfile[rd] = c.value,
      
      InsnGen::SLL(rd, rs, amount) => 
        self.regfile[rd] = self.regfile[rs] << amount.value,
      InsnGen::SRA(rd, rs, amount) => 
        self.regfile[rd] = ((self.regfile[rs] as u16) >> amount.value) as i16,
      InsnGen::SRL(rd, rs, amount) => 
        self.regfile[rd] = self.regfile[rs] >> amount.value,
      InsnGen::MOD(rd, rs, rt) => 
        self.regfile[rd] = self.regfile[rs] % self.regfile[rt],
      
      InsnGen::JMPr(rs) => {
        pc_incr = false;
        self.pc = self.regfile[rs] as u16
      },
      InsnGen::JMP(target) => self.pc = ((self.pc as i16) + target.value) as u16,
      
      InsnGen::HICONST(rd, c) => 
        self.regfile[rd] = (self.regfile[rd] & 0xFF) | ((c.value << 8) as i16),
      
      InsnGen::TRAP(target) => {
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
