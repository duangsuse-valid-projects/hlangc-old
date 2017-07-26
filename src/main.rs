#![feature(test)]

use std::fs::File;
use std::env::args;

mod parser;
mod asm;

use parser::MonkeyAST;

const VERSION: &str = "0.1.0";

fn main() {
    asm::wrap_gcc_assembly_file(MonkeyAST::new());
}
