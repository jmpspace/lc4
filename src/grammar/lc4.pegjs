
use architecture::*;
use assembler::*;
use assm_data::*;

imm16 -> IMM16
  = [-]? [1-9] [0-9]* { IMM16{value: match_str.parse().unwrap()} }
imm11 -> IMM11
  = [-]? [1-9] [0-9]* { IMM11{value: match_str.parse().unwrap()} }
imm9  -> IMM9
  = [-]? [1-9] [0-9]* { IMM9 {value: match_str.parse().unwrap()} }
imm7  -> IMM7
  = [-]? [1-9] [0-9]* { IMM7 {value: match_str.parse().unwrap()} }
imm6  -> IMM6
  = [-]? [1-9] [0-9]* { IMM6 {value: match_str.parse().unwrap()} }
imm5  -> IMM5
  = [-]? [1-9] [0-9]* { IMM5 {value: match_str.parse().unwrap()} }

uimm16 -> UIMM16
  = [1-9] [0-9]* { UIMM16{value: match_str.parse().unwrap()} }
uimm8  -> UIMM8
  = [1-9] [0-9]* { UIMM8 {value: match_str.parse().unwrap()} }
uimm7  -> UIMM7
  = [1-9] [0-9]* { UIMM7 {value: match_str.parse().unwrap()} }
uimm4  -> UIMM4
  = [1-9] [0-9]* { UIMM4 {value: match_str.parse().unwrap()} }

r_name_i -> RName
  = [0-7] { match_str.parse().unwrap() }
r_name -> RName
  = "R" a:r_name_i { a }

label -> Label
  = [A-Z] [a-zA-Z0-9_]* { match_str.to_string() }

string_s -> String
  = [^"]*  { match_str.to_string() }

string -> String
  = "\"" s:string_s "\""  { s }
  
ws -> ()
  = " "+

csws -> ()
  = "," ws
  
#[pub]
assm -> Assm
  = "NOP" { Assm::Insn(InsnGen::NOP )}
  / "BRp" ws l:label { Assm::Insn(InsnGen::BR(P, l) )}
  / "BRp" ws l:label { Assm::Insn(InsnGen::BR(Z, l) )}
  / "BRp" ws l:label { Assm::Insn(InsnGen::BR(Z|P, l) )}
  / "BRp" ws l:label { Assm::Insn(InsnGen::BR(N, l) )}
  / "BRp" ws l:label { Assm::Insn(InsnGen::BR(N|P, l) )}
  / "BRp" ws l:label { Assm::Insn(InsnGen::BR(N|Z, l) )}
  / "BRp" ws l:label { Assm::Insn(InsnGen::BR(N|Z|P, l) )}
  / "ADD" ws d:r_name csws s:r_name csws t:r_name { Assm::Insn(InsnGen::ADD(d,s,t) )}
  / "MUL" ws d:r_name csws s:r_name csws t:r_name { Assm::Insn(InsnGen::MUL(d,s,t) )}
  / "SUB" ws d:r_name csws s:r_name csws t:r_name { Assm::Insn(InsnGen::SUB(d,s,t) )}
  / "DIV" ws d:r_name csws s:r_name csws t:r_name { Assm::Insn(InsnGen::DIV(d,s,t) )}
  / "ADD" ws d:r_name csws s:r_name csws i:imm5   { Assm::Insn(InsnGen::ADDi(d,s,i) )}
  / "CMP"   ws d:r_name csws t:r_name { Assm::Insn(InsnGen::CMP(d,t) )}
  / "CMPU"  ws d:r_name csws t:r_name { Assm::Insn(InsnGen::CMPu(d,t) )}
  / "CMPI"  ws d:r_name csws i:imm7   { Assm::Insn(InsnGen::CMPi(d,i) )}
  / "CMPIU" ws d:r_name csws u:uimm7  { Assm::Insn(InsnGen::CMPiu(d,u) )}
  / "JSRR" ws s:r_name { Assm::Insn(InsnGen::JSRr(s) )}
  / "JSR"  ws l:label  { Assm::Insn(InsnGen::JSR(l) )}
  / "AND" ws d:r_name csws s:r_name csws t:r_name { Assm::Insn(InsnGen::AND(d,s,t) )}
  / "NOT" ws d:r_name csws s:r_name             { Assm::Insn(InsnGen::NOT(d,s) )}
  / "OR"  ws d:r_name csws s:r_name csws t:r_name { Assm::Insn(InsnGen::OR(d,s,t) )}
  / "XOR" ws d:r_name csws s:r_name csws t:r_name { Assm::Insn(InsnGen::XOR(d,s,t) )}
  / "AND" ws d:r_name csws s:r_name csws i:imm5   { Assm::Insn(InsnGen::ANDi(d,s,i) )}
  / "LDR" ws d:r_name csws s:r_name csws i:imm6 { Assm::Insn(InsnGen::LDR(d,s,i) )}
  / "STR" ws d:r_name csws s:r_name csws i:imm6 { Assm::Insn(InsnGen::STR(d,s,i) )}
  / "RTI" { Assm::Insn(InsnGen::RTI )}
  / "CONST" ws d:r_name csws i:imm9 { Assm::Insn(InsnGen::CONST(d,i) )}
  / "SLL" ws d:r_name csws s:r_name csws u:uimm4 { Assm::Insn(InsnGen::SLL(d,s,u) )}
  / "SRA" ws d:r_name csws s:r_name csws u:uimm4 { Assm::Insn(InsnGen::SRA(d,s,u) )}
  / "SRL" ws d:r_name csws s:r_name csws u:uimm4 { Assm::Insn(InsnGen::SRL(d,s,u) )}
  / "MOD" ws d:r_name csws s:r_name csws t:r_name { Assm::Insn(InsnGen::MOD(d,s,t) )}
  / "JMPR" ws s:r_name { Assm::Insn(InsnGen::JMPr(s) )}
  / "JMP" ws l:label { Assm::Insn(InsnGen::JMP(l) )}
  / "HICONST" ws d:r_name csws u:uimm8 { Assm::Insn(InsnGen::HICONST(d,u) )}
  / "TRAP" ws u:uimm8 { Assm::Insn(InsnGen::TRAP(u) )}
  / "RET" { Assm::RET }
  / "LEA" ws d:r_name csws l:label { Assm::LEA(d,l) }
  / "LC"  ws d:r_name csws l:label { Assm::LC(d,l) }
  / l:label { Assm::LABEL(l) }
  / ".CODE" { Assm::CODE }
  / ".DATA" { Assm::DATA }
  / ".ADDR" ws u:uimm16 { Assm::ADDR(u) }
  / ".FALIGN" { Assm::FALIGN }
  / ".FILL" ws i:imm16 { Assm::FILL(i) }
  / ".STRINGZ" ws s:string { Assm::STRINGZ(s) }
  / ".BLKW" ws u:uimm16 { Assm::BLKW(u) }
  / l:label ws ".CONST" csws i:imm16 { Assm::LCONST(l,i) }
  / l:label ws ".UCONST" csws u:uimm16 { Assm::LUCONST(l,u) }
