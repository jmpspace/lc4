
#![crate_name = "lc4"]
#![crate_type = "rlib"]
#![feature(box_syntax, core, old_io, old_path, plugin, int_uint)]
#![plugin(peg_syntax_ext)]

extern crate core;

mod architecture;
pub mod assembler;
mod controller;
mod encoder;
pub mod processor;
