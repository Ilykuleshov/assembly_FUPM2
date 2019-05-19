use std::fs::File;
use std::io::{Write, Read, SeekFrom, Seek};
use super::*;

impl CPU {
	pub fn save(&self, f: &mut File) {
		let name = "ThisIsFUPM2Exec\0".as_bytes();
		f.write_all(name).expect("Unable to write name to file!");

		macro_rules! get_bytes {
			($word:expr) => { [ 
			(($word >> 0)  as u8) & 255u8,
			(($word >> 8)  as u8) & 255u8,
			(($word >> 16) as u8) & 255u8,
			(($word >> 24) as u8) & 255u8 ]
			}
		}
		let prog_size : u32 = self.state.progsz;
		let cnst_size : u32 = 0;
		let data_size : u32 = 0;
		let begn_addr : u32 = self.state.r[15];
		let stck_addr : u32 = self.state.r[14];

		f.write_all(&get_bytes!(prog_size)).unwrap();
		f.write_all(&get_bytes!(cnst_size)).unwrap();
		f.write_all(&get_bytes!(data_size)).unwrap();
		f.write_all(&get_bytes!(begn_addr)).unwrap();
		f.write_all(&get_bytes!(stck_addr)).unwrap();

		f.seek(SeekFrom::Start(512)).unwrap();
		let prog_code = &self.state.mem[..prog_size as usize];
		for cmd in prog_code {
			f.write_all(&get_bytes!(cmd)).unwrap();
		}
	}

	pub fn load(&mut self, f: &mut File) {
		let mut name : [u8; 16] = [0; 16];
		f.read(&mut name).expect("Unable to read compiler name!");
		assert!(name == "ThisIsFUPM2Exec\0".as_bytes());

		macro_rules! get_word {
			($bytes:expr) => {
				(($bytes[0] as u32) << 0)  + 
				(($bytes[1] as u32) << 8)  + 
				(($bytes[2] as u32) << 16) + 
				(($bytes[3] as u32) << 24)
			}
		}
		let mut pars : [u8; 5 * 4] = [0; 5 * 4];
		f.read(&mut pars).expect("Unable to read pars!");

		let prog_size : u32 = get_word!(pars[0..4]);
		let cnst_size : u32 = get_word!(pars[4..8]);
		let data_size : u32 = get_word!(pars[8..12]);
		let begn_addr : u32 = get_word!(pars[12..16]);
		let stck_addr : u32 = get_word!(pars[16..20]);

		f.seek(SeekFrom::Start(512));
		self.state.r[14] = stck_addr;
		self.state.r[15] = begn_addr;
		self.state.progsz = prog_size;
		for i in 0..prog_size - 1 {
			let mut byte_arr : [u8; 4] = [0; 4];
			f.read(&mut byte_arr).expect("Unable to read line!");

			self.state.mem[i as usize] = get_word!(byte_arr);
		}
	}
}