use super::cmdspec::*;

impl CPU {
	fn docmd(&mut self, cmd: &Word) {
		let func = self.table.get_func(&((cmd & 0b11111111) as u8));
		func(&mut self.state, cmd);
	}
	pub fn exec(&mut self) {
		while !self.state.halt {
			let word = self.state.mem[self.state.r[15] as usize];
			self.docmd(&word);
			self.state.r[15] = self.state.r[15].wrapping_add(1);
		}
	}
}