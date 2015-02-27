
#![crate_name = "lc4"]
#![crate_type = "rlib"]
#![feature(box_syntax, core, old_io, old_path, plugin, int_uint)]
#![plugin(peg_syntax_ext)]

extern crate core;

pub mod architecture;
pub mod assembler;
pub mod controller;
pub mod processor;
