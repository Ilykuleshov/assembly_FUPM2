#[macro_use]
mod cpu;
mod txtparse;
mod procexec;
mod disasm;

use std::fs;

fn main() {
	let prog = fs::read_to_string("input.fasm").expect("File read error");

    let mut res = txtparse::parsecode(&prog);
    res.exec();
	
	let parsed_prog = fs::File::create("disasm.fasm").expect("Unable to create file!");
    res.disassemble(parsed_prog);
}