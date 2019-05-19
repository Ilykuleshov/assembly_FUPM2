use super::cpu::*;
use std::fs::File;
use std::io::Write;

impl CPU {
	fn disasm_cmd(&self, cmd : &Word) -> String {
		let code = &((cmd & 0b11111111) as u8);
		let name : String = self.table.get_name(code).to_owned();
		let (_, fmt) = self.table.get_code(&name);
		let args;

		match fmt {
			CmdFormat::RR   => { 
				let (r1, r2, imm) = prs!(RR => cmd);
				args = format!("r{} r{} {}", r1, r2, imm);
			}

			CmdFormat::RI   => {
				let (reg, imm) = prs!(RI => cmd);
				args = format!("r{} {}", reg, imm);
			}

			CmdFormat::RM   => {
				let (reg, mem) = prs!(RM => cmd);
				args = format!("r{} {}", reg, mem);
			}

			CmdFormat::JMEM => args = format!("{}", prs!(JM => cmd))
		}

		name + " " + &args
	}
	pub fn disassemble(&self, mut f : File) {
		let mut counter = (self.state.r[15] as usize).to_owned() as usize;
		while self.state.mem[counter] != 0 {
			let line = self.disasm_cmd(&self.state.mem[counter]) + "\n";
			f.write_all(line.as_bytes()).expect("Unable to write to file!");
			counter += 1;
		} 
	}
}