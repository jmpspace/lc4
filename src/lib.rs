
#![crate_name = "lc4"]
#![crate_type = "rlib"]
#![feature(box_syntax, plugin)]
#![plugin(peg_syntax_ext)]

//extern crate core;
extern crate byteorder;

pub mod architecture;
pub mod assembler;
pub mod assm_data;
mod controller;
mod encoder;
pub mod processor;
