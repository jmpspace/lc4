use std::convert::From;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::OpenOptions;

use architecture::*;
use assm_data::*;

#[derive(Debug)]
pub enum AssmError { IoError(io::Error), ParseError(lc4_grammar::ParseError) }

impl From<io::Error> for AssmError {
    fn from(err: io::Error) -> AssmError {
        AssmError::IoError(err)
    }
}

impl From<lc4_grammar::ParseError> for AssmError {
    fn from(err: lc4_grammar::ParseError) -> AssmError {
        AssmError::ParseError(err)
    }
}

pub type LInsn = InsnGen<Label, Label>;

#[derive(Debug)]
pub enum Assm {
    LABEL(Label),
    Insn(LInsn),
    RET,
    LEA(RName, Label),
    LC(RName, Label),
    CODE,
    DATA,
    ADDR(UIMM16),
    FALIGN,
    FILL(IMM16),
    STRINGZ(String),
    BLKW(UIMM16),
    LCONST(Label, IMM16),
    LUCONST(Label, UIMM16)
}    

peg_file! lc4_grammar("grammar/lc4.pegjs");

pub fn read_assembly_file(filename: &str) -> Result<Vec<Assm>, AssmError> {
    let mut options = OpenOptions::new();
    options.read(true);
    let file = try!(options.open(&Path::new(filename)));
    let reader = BufReader::new(file);
    let mut assms = Vec::new();
    for line in reader.lines() {

        assms.push(try!(lc4_grammar::assm(&try!(line).trim()[..])))
    }
    Ok(assms)
}

pub fn pad16(addr: u16) -> u16 {
    let mut padded = addr & 0xFFF0;
    if padded < addr { padded += 0x10; }
    padded
}

pub fn assemble(assm_lines: Vec<Assm>) -> AssmData<Mem> {

    let mut section: Section = Section::CODE;

    let mut code_addr: u16 = 0;
    let mut data_addr: u16 = 0;

    let mut addr_labels: HashMap<Label, (Section, u16)> = HashMap::new();
    let mut value_labels: HashMap<Label, i16> = HashMap::new();

    println!("First pass to place labels");

    for &ref assm in assm_lines.iter() {
        match assm {

            // Instructions and Pseudo-Instructions
            &Assm::Insn(_) => {
                assert!(section == Section::CODE);
                code_addr += 1
            },
            &Assm::RET => {
                assert!(section == Section::CODE);
                code_addr += 1
            },
            &Assm::LEA(_, _) => {
                assert!(section == Section::CODE);
                code_addr += 1
            },
            &Assm::LC(_, _) => {
                assert!(section == Section::CODE);
                code_addr += 1
            },

            // Assembler Directives
            &Assm::LABEL(ref l) => {
                if addr_labels.contains_key(l) {
                    panic!("Cannot have duplicate labels")
                }
                addr_labels.insert(l.clone(), (section, code_addr));
            },

            &Assm::CODE => section = Section::CODE,
            &Assm::DATA => section = Section::DATA,

            &Assm::ADDR(ref u) => 
                match section {
                    Section::CODE => code_addr = u.value,
                    Section::DATA => data_addr = u.value
                },

                &Assm::FALIGN => {
                    match section {
                        Section::CODE => code_addr = pad16(code_addr),
                        Section::DATA => data_addr = pad16(data_addr)
                    }
                },

                &Assm::FILL(_) => {
                    assert!(section == Section::DATA);
                    data_addr += 1
                },
                &Assm::STRINGZ(ref s) => {
                    assert!(section == Section::DATA);
                    data_addr += s.len() as u16
                },

                &Assm::BLKW(ref u) => {
                    match section {
                        Section::CODE => code_addr += u.value,
                        Section::DATA => data_addr += u.value
                    }
                },

                &Assm::LCONST(ref l, ref i) => {
                    if value_labels.contains_key(l) {
                        panic!("Cannot have duplicate labels")
                    }
                    value_labels.insert(l.clone(), i.value);
                }
            &Assm::LUCONST(ref l, ref u) => {
                if value_labels.contains_key(l) {
                    panic!("Cannot have duplicate labels")
                }
                value_labels.insert(l.clone(), u.value as i16);
            }
        }
    }

    println!("Second pass to place instructions");

    let base_data_addr = pad16(code_addr);
    let base_heap_addr = pad16(base_data_addr + data_addr);
    let mut memory: Memory<Mem> = box [Mem::DATA(0);0x10000];
    let mut addr: u16 = 0;

    println!("heap {}", base_heap_addr);

    for &ref assm in assm_lines.iter() {
        println!("PC {} Insn {:?}", addr, assm);

        match assm {

            &Assm::LABEL(ref target) => {
                let (label_section, label_addr) = addr_labels[&target.clone()];
                match label_section {
                    Section::CODE => addr = label_addr,
                    Section::DATA => addr = label_addr + base_data_addr
                }
            },

            &Assm::Insn(InsnGen::BR(cc, ref target)) => {
                let (section, label_addr) = addr_labels[&target.clone()];
                assert!(section == Section::CODE);
                memory[addr as usize] = Mem::CODE(InsnGen::BR(cc, IMM9{value: (label_addr - (addr + 1)) as i16}));
                addr += 1
            },
            &Assm::Insn(InsnGen::JSR(ref target)) => {
                let (section, label_addr) = addr_labels[&target.clone()];
                assert!(section == Section::CODE);
                memory[addr as usize] = Mem::CODE(InsnGen::JSR(IMM11{value: (label_addr - (addr & 0x8000)) as i16 >> 4}));
                addr += 1
            },
            &Assm::Insn(InsnGen::JMP(ref target)) => {
                let (section, label_addr) = addr_labels[&target.clone()];
                assert!(section == Section::CODE);
                memory[addr as usize] = Mem::CODE(InsnGen::JMP(IMM11{value: (label_addr - (addr + 1)) as i16}));
                addr += 1
            },

            &Assm::Insn(ref insn) => {
                memory[addr as usize] = Mem::CODE(partial_cast(insn));
                addr += 1
            }

            &Assm::RET => {
                memory[addr as usize] = Mem::CODE(InsnGen::JMPr(R7));
                addr += 1
            },

            &Assm::LEA(rd, ref target) => {
                let (section, label_addr) = addr_labels[&target.clone()];
                let label_addr = match section {
                    Section::CODE => label_addr,
                    Section::DATA => label_addr + base_data_addr
                };
                let low = IMM9{value: label_addr as i16 & 0x01FF};
                let high = UIMM8{value: label_addr >> 8};
                memory[addr as usize] = Mem::CODE(InsnGen::CONST(rd, low));
                memory[addr as usize + 1] = Mem::CODE(InsnGen::HICONST(rd, high));
                addr += 2
            },

            &Assm::LC(rd, ref target) => {
                let label_value = value_labels[&target.clone()];
                let low = IMM9{value: label_value & 0x01FF};
                let high = UIMM8{value: label_value as u16 >> 8};
                memory[addr as usize] = Mem::CODE(InsnGen::CONST(rd, low));
                memory[addr as usize + 1] = Mem::CODE(InsnGen::HICONST(rd, high));
                addr += 2
            }

            &Assm::CODE => (),
            &Assm::DATA => (),
            &Assm::ADDR(_) => (),
            &Assm::FALIGN => (),

            &Assm::FILL(i) => {
                memory[addr as usize] = Mem::DATA(i.value);
                addr += 1
            },

            &Assm::STRINGZ(_) => panic!("Not implemented"),

            &Assm::BLKW(_) => (),
            &Assm::LCONST(_,_) => (),
            &Assm::LUCONST(_,_) => ()
        }
    }

    AssmData{
        memory: memory,
        labels: addr_labels,
        heap: base_heap_addr
    }
}
