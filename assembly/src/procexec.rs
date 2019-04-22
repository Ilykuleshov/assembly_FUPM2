use super::cmdspec::*;

impl CPU {
	fn docmd(&mut self, cmd: &Word) {
		let code = &((cmd & 0b11111111) as u8);
		if self.state.mode & dbmode::CMD != 0 {
			let name = self.table.get_name(code);
			println!("CMD=({})", name);
			if self.state.mode & dbmode::ARG != 0 {
				let (_, fmt) = self.table.get_code(name);
				let args;
				match fmt {
					CmdFormat::RR   => args = format!("{:?}", prs!(RR => cmd)),
					CmdFormat::RI   => args = format!("{:?}", prs!(RI => cmd)),
					CmdFormat::RM   => args = format!("{:?}", prs!(RM => cmd)),
					CmdFormat::JMEM => args = format!("{:?}", prs!(JM => cmd))
				}

				println!("ARGS=({})", args);
			}
		}

		let func = self.table.get_func(code);
		func(&mut self.state, cmd);

		if self.state.mode & dbmode::REG != 0 {
			print!("REG=");
			for i in 0..16 {
				if i == 14 {
					print!("(STPTR={})", MEMSZ as u32 - self.state.r[i]);
				}
				print!("{}:{} | ", i, self.state.r[i]);

			}
			print!("\n\n")
		}
	}
	pub fn exec(&mut self) {
		while !self.state.halt {
			let word = self.state.mem[self.state.r[15] as usize];
			self.docmd(&word);
			self.state.r[15] = self.state.r[15].wrapping_add(1);
		}
	}
}