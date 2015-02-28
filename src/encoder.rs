
use architecture::*;

pub fn encode_insn(insn: Insn) -> i16 {
  match insn {
    InsnGen::NOP => 0x0000,
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
    

    _ => panic!("Not imeplemented")
  }
}