mod txtparse;
mod cmdspec;
mod procexec;

use std::fs;

fn main() {
	let prog = fs::read_to_string("prog.txt").expect("File read error");
    let mut res = txtparse::parsecode(&prog);
    res.exec();
}