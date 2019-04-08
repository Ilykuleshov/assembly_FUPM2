use std::collections::HashMap;
use std::vec::Vec;

pub type Word = u32;

#[derive(Clone)]
pub enum CmdFormat { RM, RR, RI }

type Func = &'static Fn(&mut CpuState, &Word);

pub struct CmdTable {
    code: HashMap<&'static str, (u8, CmdFormat)>,
    func: HashMap<u8, Func>
}

use CmdFormat::*;

impl CmdTable {
    pub fn insert(&mut self, name: &'static str, code: u8, format: CmdFormat, func: Func) {
        self.code.insert(name, (code, format));
        self.func.insert(code, func);
    }

    pub fn get_code(&self, name: &str) -> &(u8, CmdFormat) { self.code.get(name).unwrap() }
    pub fn get_func (&self, code: &u8) -> &Func { self.func.get(code).unwrap() }

    pub fn new() -> CmdTable {
        let mut table = CmdTable {
            code: HashMap::new(),
            func: HashMap::new()
        };

        macro_rules! RR {
            ($word:expr) => { (
                (($word >> 8)  & 0b1111u32), 
                (($word >> 12) & 0b1111u32), 
                ($word >> 16)
                ) }
        }

        macro_rules! RM {
            ($word:expr) => { (
                (($word >> 8) & 0b1111u32), 
                ($word >> 12)
                ) }
        }

        macro_rules! RI {
            ($word:expr) => { (
                (($word >> 8) & 0b1111u32), 
                ($word >> 12u32)
                ) }
        }

        table.insert("halt", 0, RI, &|cpu, _| cpu.halt = true);
        table.insert("syscall", 1, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            match imm {
                0 => cpu.halt = true,
                102 => println!("{}", cpu.r[reg as usize]),
                _ => panic!("Bad syscall argument!")
            }
        });
/*        table.insert("add", 2, RR, &|cpu, arg| {
            let (r1, r2, imm) = RR!(arg);

            cpu.r[r1] = cpu.r[r2] + imm;
        });
        table.insert("addi", 3, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            cpu.r[reg] = cpu.r[reg] + imm;
        });*/

        table
    }
}

pub const MEMSZ : usize = 1 << 20;

pub struct CpuState {
    pub mem: Vec<Word>,
    pub r: [Word; 16],
    pub halt: bool
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState{ mem: vec![0; MEMSZ], r: [0; 16], halt: false }
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