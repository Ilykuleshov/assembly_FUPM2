#[macro_use]
mod cpu;
mod txtparse;
mod procexec;
mod disasm;

use std::{fs, fs::File};

fn main() {
	// let prog = fs::read_to_string("input.fasm").expect("File read error");
    // let mut f = File::create("binary.basm").expect("Unable to open file for reading!");
    // let mut res = txtparse::parsecode(&prog);
    // res.save(&mut f);
 	let mut f = File::open("exec.fbin").expect("Unable to open file for reading!");
 	let mut res = cpu::CPU::new();
    res.load(&mut f);
	let parsed_prog = fs::File::create("disasm.fasm").expect("Unable to create file!");
    res.disassemble(parsed_prog);
    // res.exec()
}