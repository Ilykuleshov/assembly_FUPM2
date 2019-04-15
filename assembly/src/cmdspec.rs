use std::collections::HashMap;
use std::vec::Vec;
use std::io;

pub type Word = u32;
pub type DWord = u64;

#[derive(Clone)]
pub enum CmdFormat { RM, RR, RI, JMEM }

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

    pub fn get_code(&self, name: &str) -> &(u8, CmdFormat) { self.code.get(name).expect(&format!("Bad cmd name! ({})", name)) }
    pub fn get_func (&self, code: &u8) -> &Func { self.func.get(code).expect(&format!("Bad cmd code! ({})", code)) }

    pub fn new() -> CmdTable {
        let mut table = CmdTable {
            code: HashMap::new(),
            func: HashMap::new()
        };

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

        table.insert("halt", 0, RI, &|cpu, _| cpu.halt = true);
        table.insert("syscall", 1, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

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
            let (r1, r2, imm) = prs!(RR => arg);

            cpu.r[r1] += cpu.r[r2].wrapping_add(imm);
        });
        table.insert("addi", 3, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            cpu.r[reg] = cpu.r[reg].wrapping_add(imm);
        });
        table.insert("sub", 4, RR, &|cpu, arg| {
            let (r1, r2, imm) = prs!(RR => arg);

            cpu.r[r1] -= cpu.r[r2].wrapping_add(imm);
        });
        table.insert("subi", 5, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            cpu.r[reg] = cpu.r[reg].wrapping_sub(imm);
        });
        table.insert("mul", 6, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            let mul: DWord = (cpu.r[r1] as u64) * (cpu.r[r2] as u64);
            cpu.r[r1] = (mul & 0b11111111111111111111111111111111) as u32;
            cpu.r[r1 + 1] = (mul >> 32) as u32;
        });
        table.insert("muli", 7, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            let mul: DWord = (cpu.r[reg] as u64) * (imm as u64);
            cpu.r[reg] = (mul & 0b11111111111111111111111111111111) as u32;
            cpu.r[reg + 1] = (mul >> 32) as u32;
        });
        table.insert("div", 8, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            let q: u32 = cpu.r[r1] / cpu.r[r2];
            let r: u32 = cpu.r[r1] % cpu.r[r2];

            cpu.r[r1] = q;
            cpu.r[r1 + 1] = r;
        });
        table.insert("divi", 9, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            let r = cpu.r[reg] % imm;
            cpu.r[reg] /= imm;
            cpu.r[reg + 1] = r;
        });
        table.insert("lc", 12, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            cpu.r[reg] = imm;
        });
        table.insert("shl", 13, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            cpu.r[r1] <<= cpu.r[r2];
        });
        table.insert("shli", 14, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            cpu.r[reg] <<= imm;
        });
        table.insert("shr", 15, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            cpu.r[r1] >>= cpu.r[r2]
        });
        table.insert("shri", 16, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            cpu.r[reg] >>= imm;
        });
        table.insert("and", 17, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            cpu.r[r1] &= cpu.r[r2];
        });
        table.insert("andi", 18, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            cpu.r[reg] &= imm;
        });
        table.insert("cmp", 43, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            if cpu.r[r1] == cpu.r[r2] {
                cpu.f = Flag::E;
            } else
            if cpu.r[r1] >  cpu.r[r2] {
                cpu.f = Flag::G;
            } else {
                cpu.f = Flag::L;
            }
        });

        impl CpuState {
            pub fn jump(&mut self, adr: u32) {
                self.r[15] = adr.wrapping_sub(1);
            }
        }

        macro_rules! insert_jmp {
            ($name:expr, $num:expr, $rel:tt, $obj:expr) => {
                table.insert($name, $num, JMEM, &|cpu, arg| {
                    let mem = prs!(JM => arg);
                    if cpu.f $rel $obj {
                        cpu.jump(mem);
                    }
                });
            };
        }
        
        insert_jmp!("jmp", 46, !=, Flag::NAN);
        insert_jmp!("jne", 47, !=, Flag::E);
        insert_jmp!("jeq", 48, ==, Flag::E);
        insert_jmp!("jle", 49, !=, Flag::G);
        insert_jmp!("jl",  50, ==, Flag::L);
        insert_jmp!("jge", 51, !=, Flag::L);
        insert_jmp!("jg",  52, ==, Flag::G);

        table
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

pub struct CpuState {
    pub mem: Vec<Word>,
    pub r: [Word; 16],
    pub f : Flag,
    pub halt: bool
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState{ mem: vec![0; MEMSZ], r: [0; 16], f : Flag::NAN, halt: false }
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