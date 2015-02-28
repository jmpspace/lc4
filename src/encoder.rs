
use architecture::*;

pub fn encode_insn(insn: Insn) -> i16 {
  match insn {
    InsnGen::NOP => 
      (0x0000 << 12),
    InsnGen::BR(cc, i) =>
      (0b0000 << 12) | ((cc as i16 & 0x7) << 9) | (i.value & 0x01FF),

    InsnGen::ADD(rd, rs, rt) => 
      (0b0001 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b000 << 3) | (rt as i16 & 0x7),
    InsnGen::MUL(rd, rs, rt) => 
      (0b0001 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b001 << 3) | (rt as i16 & 0x7),
    InsnGen::SUB(rd, rs, rt) => 
      (0b0001 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b010 << 3) | (rt as i16 & 0x7),
    InsnGen::DIV(rd, rs, rt) => 
      (0b0001 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b011 << 3) | (rt as i16 & 0x7),
    InsnGen::ADDi(rd, rs, i) =>
      (0b0001 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b1 << 5) | (i.value & 0x1F),
    
    InsnGen::CMP(rd, rt) =>
      (0b0010 << 12) | ((rd as i16 & 0x7) << 9) | (0b00 << 7) | (rt as i16 & 0x7),
    InsnGen::CMPu(rd, rt) =>
      (0b0010 << 12) | ((rd as i16 & 0x7) << 9) | (0b01 << 7) | (rt as i16 & 0x7),
    InsnGen::CMPi(rd, i) =>
      (0b0010 << 12) | ((rd as i16 & 0x7) << 9) | (0b10 << 7) | (i.value & 0x7F),
    InsnGen::CMPiu(rd, u) =>
      (0b0010 << 12) | ((rd as i16 & 0x7) << 9) | (0b11 << 7) | (u.value as i16 & 0x7F),

    InsnGen::JSR(i) =>
      (0b01001 << 11) | (i.value & 0x7FF),
    InsnGen::JSRr(rs) =>
      (0b01000 << 11) | ((rs as i16 & 0x7) << 6),

    InsnGen::AND(rd, rs, rt) => 
      (0b0101 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b000 << 3) | (rt as i16 & 0x7),
    InsnGen::NOT(rd, rs) => 
      (0b0101 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b001 << 3),
    InsnGen::OR(rd, rs, rt) => 
      (0b0101 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b010 << 3) | (rt as i16 & 0x7),
    InsnGen::XOR(rd, rs, rt) => 
      (0b0101 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b011 << 3) | (rt as i16 & 0x7),
    InsnGen::ANDi(rd, rs, i) =>
      (0b0101 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b1 << 5) | (i.value & 0x1F),

    InsnGen::LDR(rd, rs, i) =>
      (0b0110 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (i.value & 0x3F),
    InsnGen::STR(rd, rs, i) =>
      (0b0111 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (i.value & 0x3F),
    
    InsnGen::RTI =>
      (0b1000 << 12),

    InsnGen::CONST(rd, i) =>
      (0b1001 << 12) | ((rd as i16 & 0x7) << 9) | (i.value & 0x01FF),

    InsnGen::SLL(rd, rs, u) =>
      (0b1010 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b00 << 4) | (u.value as i16 & 0xF),
    InsnGen::SRA(rd, rs, u) =>
      (0b1010 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b01 << 4) | (u.value as i16 & 0xF),
    InsnGen::SRL(rd, rs, u) =>
      (0b1010 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b10 << 4) | (u.value as i16 & 0xF),
    InsnGen::MOD(rd, rs, rt) => 
      (0b1010 << 12) | ((rd as i16 & 0x7) << 9) | ((rs as i16 & 0x7) << 6) | (0b11 << 4) | (rt as i16 & 0x7),

    InsnGen::JMPr(rs) =>
      (0b11000 << 11) | ((rs as i16 & 0x7) << 6),
    InsnGen::JMP(i) =>
      (0b11001 << 11) | (i.value & 0x7FF),

    InsnGen::HICONST(rd, u) =>
      (0b1101 << 12) | ((rd as i16 & 0x7) << 9) | (0b1 << 8) | (u.value as i16 & 0xFF),

    InsnGen::TRAP(u) =>
      (0b1111 << 12) | (u.value as i16 & 0xFF)
  }
}