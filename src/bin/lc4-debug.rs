extern crate lc4;

use std::cmp::*;
use std::env::args;
use std::io::*;

use lc4::assm_data::*;
use lc4::processor::*;

fn print_proc(cpu: &CPU) -> () {
  println!("Registers {:?} NZP {} PC {}", cpu.regfile, cpu.nzp, cpu.pc);
  let radius = 3;
  let low = max(0, cpu.pc as i32 - radius) as usize;
  let high = min(cpu.pc as i32 + radius + 1, cpu.memory.len() as i32) as usize;
  println!("{} {}", low, high);
  for i in (low .. high) {
    println!("{} {:#04x} {}", if i == cpu.pc as usize {"*"} else {" "}, i, cpu.memory[i]);
  }
}

pub fn main() -> () {
  let ref source_file: String = match args().nth(1) {
    Some(arg) => arg,
    None => {
      println!("Missing source file argument");
      return
    }
  };

  let assm_data: AssmData<i16> = match read_object_file(source_file) {
    Err(err) => panic!("{:?}", err),
    Ok(data) => data
  };

  let mut cpu = boot(assm_data);

  print_proc(&cpu);

  let stdin = stdin();

  for liner in stdin.lock().lines() {
    match liner {
      Err(err) => panic!("{:?}", err),
      Ok(line) => match line.trim() {
        "p" => print_proc(&cpu),
        "s" => match cpu.step() {
          Err(err) => panic!("{:?}", err),
          Ok(()) => ()
        },
        "q" => break,
        _ => continue
      }
    }
    
  }
  
}
