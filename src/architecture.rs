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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMM16 { pub value : i16 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMM11 { pub value : i16 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMM9 { pub value : i16 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMM7 { pub value : i16 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMM6 { pub value : i16 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMM5 { pub value : i16 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UIMM16 { pub value : u16 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UIMM8 { pub value : u16 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UIMM7 { pub value : u16 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UIMM4 { pub value : u16 }

// Insnructions

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsnGen<BrT, JmpT> {
  NOP,
  BR(CC, BrT),
  ADD(RName, RName, RName),
  MUL(RName, RName, RName),
  SUB(RName, RName, RName),
  DIV(RName, RName, RName),
  ADDi(RName, RName, IMM5),
  CMP(RName, RName),
  CMPu(RName, RName),
  CMPi(RName, IMM7),
  CMPiu(RName, UIMM7),
  JSR(JmpT),
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
  JMP(JmpT),
  HICONST(RName, UIMM8),
  TRAP(UIMM8)
}

pub type Insn = InsnGen<IMM9, IMM11>;

pub fn partial_cast<A,B,C,D>(insn: &InsnGen<A,B>) -> InsnGen<C,D> {
  match insn {
    &InsnGen::NOP               => InsnGen::NOP,
    &InsnGen::BR(_,_)           => panic!("Partial case fails for BR"),
    &InsnGen::ADD(rd, rs, rt)   => InsnGen::ADD(rd, rs, rt),
    &InsnGen::MUL(rd, rs, rt)   => InsnGen::MUL(rd, rs, rt),
    &InsnGen::SUB(rd, rs, rt)   => InsnGen::SUB(rd, rs, rt),
    &InsnGen::DIV(rd, rs, rt)   => InsnGen::DIV(rd, rs, rt),
    &InsnGen::ADDi(rd, rs, rt)  => InsnGen::ADDi(rd, rs, rt),
    &InsnGen::CMP(rd, rt)       => InsnGen::CMP(rd, rt),
    &InsnGen::CMPu(rd, rt)      => InsnGen::CMPu(rd, rt),
    &InsnGen::CMPi(rd, i)       => InsnGen::CMPi(rd, i),
    &InsnGen::CMPiu(rd, u)      => InsnGen::CMPiu(rd, u),
    &InsnGen::JSR(_)            => panic!("Partial case fails for JSR"),
    &InsnGen::JSRr(rs)          => InsnGen::JSRr(rs),
    &InsnGen::AND(rd, rs, rt)   => InsnGen::AND(rd, rs, rt),
    &InsnGen::NOT(rd, rs)       => InsnGen::NOT(rd, rs),
    &InsnGen::OR(rd, rs, rt)    => InsnGen::OR(rd, rs, rt),
    &InsnGen::XOR(rd, rs, rt)   => InsnGen::XOR(rd, rs, rt),
    &InsnGen::ANDi(rd, rs, i)   => InsnGen::ANDi(rd, rs, i),
    &InsnGen::LDR(rd, rs, i)    => InsnGen::LDR(rd, rs, i),
    &InsnGen::STR(rd, rs, i)    => InsnGen::STR(rd, rs, i),
    &InsnGen::RTI               => InsnGen::RTI,
    &InsnGen::CONST(rd, i)      => InsnGen::CONST(rd, i),
    &InsnGen::SLL(rd, rs, u)    => InsnGen::SLL(rd, rs, u),
    &InsnGen::SRA(rd, rs, u)    => InsnGen::SRA(rd, rs, u),
    &InsnGen::SRL(rd, rs, u)    => InsnGen::SRL(rd, rs, u),
    &InsnGen::MOD(rd, rs, rt)   => InsnGen::MOD(rd, rs, rt),
    &InsnGen::JMPr(rs)          => InsnGen::JMPr(rs),
    &InsnGen::JMP(_)            => panic!("Partial case fails for JMP"),
    &InsnGen::HICONST(rd, u)    => InsnGen::HICONST(rd, u),
    &InsnGen::TRAP(u)           => InsnGen::TRAP(u)
  }
}
