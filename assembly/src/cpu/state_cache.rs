use std::fs::File;
use std::io::Write;
use super::*;

impl CpuState {
	fn save(&self, f: &mut File) {
		let name = "ThisIsFUPM2Exec".as_bytes();
		f.write_all(name).expect("Unable to write to file!");
	}
}