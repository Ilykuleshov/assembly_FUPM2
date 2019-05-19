use super::cpu::*;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

impl CPU {
	fn disasm_cmd(&self, cmd : &Word, labeltbl: &HashMap<u32, u32>) -> String {
		if *cmd == 0 { "word".to_string(); }
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

			CmdFormat::JMEM => {
				let mem = prs!(JM => cmd);
				match labeltbl.get(&mem) {
					Some(labelnum) => args = format!("label{}", labelnum),
					None           => args = format!("{}", mem)
				}
			}
		}

		name + " " + &args
	}
	pub fn disassemble(&self, mut f : File) {
		let mut labeltbl : HashMap<u32, u32> = HashMap::new();
		let mut labelcnt = 1;

		for i in 0..self.state.progsz {
			let line = self.state.mem[i as usize];
				let code = getcode!(line);
				if code == 41 {
					let (_, mem) = prs!(RM => line);
					labeltbl.insert(mem, labelcnt);
					labelcnt += 1;
			}
		}

		labeltbl.insert(self.state.r[15], 0);
		for  i in 0..self.state.progsz {
			if labeltbl.contains_key(&i) {
				let lbl_str = format!("label{}:\n", labeltbl.get(&i).expect("Label error!"));
				f.write_all(lbl_str.as_bytes()).unwrap();
			}
			let word = self.state.mem[i as usize];
			let line = self.disasm_cmd(&word, &labeltbl) + "\n";
			f.write_all(line.as_bytes()).unwrap();
		}

		f.write_all(b"end label0").unwrap();
	}
}