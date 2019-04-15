use std::collections::HashMap;
use std::vec::Vec;
use std::io;

pub type Word = u32;
pub type DWord = u64;

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
                (($word >> 8)  & 0b1111u32) as usize, 
                (($word >> 12) & 0b1111u32) as usize, 
                (($word.clone() as i32) >> 16) as u32
                ) }
        }

        macro_rules! RM {
            ($word:expr) => { (
                (($word >> 8) & 0b1111u32) as usize, 
                ($word >> 12)
                ) }
        }

        macro_rules! RI {
            ($word:expr) => { (
                (($word >> 8) & 0b1111u32) as usize, 
                (($word.clone() as i32) >> 12) as u32
                ) }
        }

        table.insert("halt", 0, RI, &|cpu, _| cpu.halt = true);
        table.insert("syscall", 1, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            match imm {
                0 => cpu.halt = true,
                100 => {
                    let mut in_txt = String::new();
                    io::stdin().read_line(&mut in_txt).expect("Failed to read from stdin");
                    cpu.r[reg] = in_txt.trim().parse::<u32>().expect("Failed to parse stdin");
                },
                102 => println!("{}", cpu.r[reg]),
                _ => panic!("Bad syscall argument!")
            }
        });
        table.insert("add", 2, RR, &|cpu, arg| {
            let (r1, r2, imm) = RR!(arg);

            cpu.r[r1] += cpu.r[r2].wrapping_add(imm);
        });
        table.insert("addi", 3, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            cpu.r[reg] = cpu.r[reg].wrapping_add(imm);
        });
        table.insert("sub", 4, RR, &|cpu, arg| {
            let (r1, r2, imm) = RR!(arg);

            cpu.r[r1] -= cpu.r[r2].wrapping_add(imm);
        });
        table.insert("subi", 5, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            cpu.r[reg] = cpu.r[reg].wrapping_sub(imm);
        });
        table.insert("mul", 6, RR, &|cpu, arg| {
            let (r1, r2, _) = RR!(arg);

            let mul: DWord = (cpu.r[r1] as u64) * (cpu.r[r2] as u64);
            cpu.r[r1] = (mul & 0b11111111111111111111111111111111) as u32;
            cpu.r[r1 + 1] = (mul >> 32) as u32;
        });
        table.insert("muli", 7, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            let mul: DWord = (cpu.r[reg] as u64) * (imm as u64);
            cpu.r[reg] = (mul & 0b11111111111111111111111111111111) as u32;
            cpu.r[reg + 1] = (mul >> 32) as u32;
        });
        table.insert("div", 8, RR, &|cpu, arg| {
            let (r1, r2, _) = RR!(arg);

            let q: u32 = cpu.r[r1] / cpu.r[r2];
            let r: u32 = cpu.r[r1] % cpu.r[r2];

            cpu.r[r1] = q;
            cpu.r[r1 + 1] = r;
        });
        table.insert("divi", 9, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            let r = cpu.r[reg] % imm;
            cpu.r[reg] /= imm;
            cpu.r[reg + 1] = r;
        });
        table.insert("lc", 12, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            cpu.r[reg] = imm;
        });
        table.insert("shl", 13, RR, &|cpu, arg| {
            let (r1, r2, _) = RR!(arg);

            cpu.r[r1] <<= cpu.r[r2];
        });
        table.insert("shli", 14, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            cpu.r[reg] <<= imm;
        });
        table.insert("shr", 15, RR, &|cpu, arg| {
            let (r1, r2, _) = RR!(arg);

            cpu.r[r1] >>= cpu.r[r2]
        });
        table.insert("shri", 16, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            cpu.r[reg] >>= imm;
        });
        table.insert("and", 17, RR, &|cpu, arg| {
            let (r1, r2, _) = RR!(arg);

            cpu.r[r1] &= cpu.r[r1];
        })
        table.insert("andi", 18, RI, &|cpu, arg| {
            let (reg, imm) = RI!(arg);

            cpu.r[reg] &= imm;
        });
        table.insert("jmp", 46, RM, &|cpu, arg| {
            let (_, mem) = RM!(arg);

            cpu.r[15] = mem;
        });

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