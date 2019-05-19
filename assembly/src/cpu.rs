use std::collections::HashMap;
use std::vec::Vec;

macro_rules! prs {
    (RR => $word:expr) => { (
        (($word >> 8)  & 0b1111u32) as usize, 
        (($word >> 12) & 0b1111u32) as usize, 
        (($word.clone() as i32) >> 16) as u32
        ) };
    (RM => $word:expr) => { (
        (($word >> 8) & 0b1111u32) as usize, 
        ($word >> 12)
        ) };
    (RI => $word:expr) => { (
        (($word >> 8) & 0b1111u32) as usize, 
        (($word.clone() as i32) >> 12) as u32
        ) };
    (JM => $word:expr) => { {
        let (_, mem) : (_, u32) = prs!(RM => $word);
        mem
        } };
}

mod cmdspec;
mod cpu_impl;

pub type Word = u32;
pub type DWord = u64;

#[derive(Clone)]
pub enum CmdFormat { RM, RR, RI, JMEM }

type Func = &'static Fn(&mut CpuState, &Word);

pub struct CmdTable {
    code: HashMap<&'static str, (u8, CmdFormat)>,
    func: HashMap<u8, Func>,
    name: HashMap<u8, &'static str>
}

impl CmdTable {
    pub fn insert(&mut self, name: &'static str, code: u8, format: CmdFormat, func: Func) {
        self.code.insert(name, (code, format));
        self.func.insert(code, func);
        self.name.insert(code, name);
    }

    pub fn get_code(&self, name: &str) -> &(u8, CmdFormat) {
        self.code.get(name)
            .expect(&format!("Bad cmd name! ({})", name)) 
    }
    pub fn get_func(&self, code: &u8) -> &Func {
        self.func.get(code)
            .expect(&format!("Bad cmd code! ({})", code))
    }
    pub fn get_name(&self, code: &u8)  -> &str {
        self.name.get(code)
            .expect(&format!("Bad cmd code! ({})", code))
    }

}

pub const MEMSZ : usize = 1 << 20;

#[derive(Clone, Copy)]
pub enum Flag { NAN = 0, G = 1, E = 2, L = 3 }
impl PartialEq for Flag {
    fn eq(&self, other: &Flag) -> bool {
        if *self as u32 == Flag::NAN as u32 || *other as u32 == Flag::NAN as u32 {
            false
        }
        else { *self as u32 == *other as u32 }
    }
}

pub mod dbmode {
    pub const CMD: u8 = 0b001;
    pub const ARG: u8 = 0b010;
    pub const REG: u8 = 0b100;   
}

pub struct CpuState {
    pub mem: Vec<Word>,
    pub r: [Word; 16],
    pub f : Flag,
    pub halt: bool,
    pub mode: u8
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState{ mem: vec![0; MEMSZ], r: [0; 16], f : Flag::NAN, halt: false, mode: 0 }
    }
}

pub struct CPU {
    pub state: CpuState,
    pub table: CmdTable
}

impl CPU {
    pub fn new() -> CPU {
        CPU{ state: CpuState::new(), table: CmdTable::new() }
    }
}