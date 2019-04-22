#[macro_use]
mod cmdspec;
mod txtparse;
mod procexec;

use std::fs;

fn main() {
	let prog = fs::read_to_string("input.fasm").expect("File read error");
    let mut res = txtparse::parsecode(&prog);
    res.exec();
}