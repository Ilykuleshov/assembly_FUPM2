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

        use std::f64;
        macro_rules! convd {
            ($fsti:expr, $sndi:expr) => { {
                let num : u64 = $fsti as u64 + (($sndi as u64) << 32);
                f64::from_bits(num)
            } };
            ($float:expr) => { {
                let num : u64 = $float.to_bits();
                let fst : u32 = (num & 0b11111111111111111111111111111111) as u32;
                let snd : u32 = (num << 32) as u32;

                (fst, snd)
            } };
        }

        macro_rules! insert {
            (op => $name:expr, $num:expr, $op:tt) => {{
                table.insert($name, $num, RR, &|cpu, arg| {
                    let (r1, r2, imm) = prs!(RR => arg);

                    cpu.r[r1] $op cpu.r[r2].wrapping_add(imm);
                })
            }};
            (opi => $name:expr, $num:expr, $op:tt) => {{
                table.insert($name, $num, RI, &|cpu, arg| {
                    let (reg, imm) = prs!(RI => arg);

                    cpu.r[reg] $op imm;
                });
            }};
            (opd => $name:expr, $num:expr, $op:tt) => {{
                table.insert($name, $num, RR, &|cpu, arg| {
                    let (r1, r2, _) = prs!(RR => arg);

                    let f1 = cpu.scand(r1);
                    let f2 = cpu.scand(r2);
                    cpu.writed(f1 $op f2, r1);
                });
            }};
            (jmp => $name:expr, $num:expr, $rel:tt, $obj:expr) => {
                table.insert($name, $num, JMEM, &|cpu, arg| {
                    let mem = prs!(JM => arg);
                    if cpu.f $rel $obj {
                        cpu.jump(mem);
                    }
                });
            };
        }

        impl CpuState {
            fn scand(&self, reg: usize) -> f64 {
                convd!(self.r[reg], self.r[reg + 1])
            }

            fn writed(&mut self, f: f64, reg: usize) {
                let (u1, u2) = convd!(f);
                self.r[reg] = u1;
                self.r[reg + 1] = u2;
            }

            fn push(&mut self, val: Word) {
                self.mem[self.r[14] as usize] = val;
                self.r[14] -= 1;
            }

            fn pop(&mut self) -> u32 {
                let ret = self.mem[self.r[14] as usize];
                self.r[14] += 1;
                ret
            }

            fn cmp<T:PartialOrd>(&mut self, val1: T, val2: T) {
                if val1 < val2 {
                    self.f = Flag::L;
                } else
                if val1 > val2 {
                    self.f = Flag::G; 
                } else {
                    self.f = Flag::E;
                }
            }

            fn jump(&mut self, adr: u32) {
                self.r[15] = adr.wrapping_sub(1);
            }
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

        insert!(op => "add", 2, +=);
        table.insert("addi", 3, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            cpu.r[reg] = cpu.r[reg].wrapping_add(imm);
        });
        insert!(op => "sub", 4, -=);
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
        insert!(opi => "lc",   12, =);
        insert!(op  => "shl",  13, <<=);
        insert!(opi => "shli", 14, <<=);
        insert!(op  => "shr",  15, >>=);
        insert!(opi => "shri", 16, >>=);
        insert!(op  => "and",  17, &=);
        insert!(opi => "andi", 18, &=);
        insert!(op  => "or",   19, |=);
        insert!(opi => "ori",  20, |=);
        insert!(op  => "xor",  21, ^=);
        insert!(opi => "xori", 22, ^=);
        table.insert("not", 23, RI, &|cpu, arg| {
            let (reg, _) = prs!(RI => arg);

            cpu.r[reg] = !cpu.r[reg];
        });
        insert!(op  => "mov",  24, =);

        insert!(opd => "addd", 32, +);
        insert!(opd => "subd", 33, -);
        insert!(opd => "muld", 34, *);
        insert!(opd => "divd", 35, /);
        
        table.insert("itod", 36, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            cpu.writed(cpu.r[r2] as f64, r1);
        });
        table.insert("dtoi", 37, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            let src = cpu.scand(r2).trunc();
            let res;
            if src < 0.0 { res = (src as i32) as u32; }
            else         { res = src as u32; }

            cpu.r[r1] = res;
        });

        table.insert("push", 38, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);
            cpu.push(cpu.r[reg] + imm);
        });

        table.insert("pop", 39, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);
            cpu.r[reg] = cpu.pop() + imm;
        });

        table.insert("call", 40, RR, &|cpu, arg| {
            let (r1, r2, imm) = prs!(RR => arg);

            cpu.push(cpu.r[15]);
            cpu.r[r1] = cpu.r[15];
            cpu.jump(cpu.r[r2] + imm);
        });

        table.insert("calli", 41, RI, &|cpu, arg| {
            
        });

        table.insert("cmp", 43, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            cpu.cmp(cpu.r[r1], cpu.r[r2]);
        });
        table.insert("cmpi", 44, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            cpu.cmp(cpu.r[reg], imm);
        });
        table.insert("cmpd", 45, RR, &|cpu, arg| {
            let (r1, r2, _) = prs!(RR => arg);

            cpu.cmp(cpu.scand(r1), cpu.scand(r2));
        });
        
        insert!(jmp => "jmp", 46, !=, Flag::NAN);
        insert!(jmp => "jne", 47, !=, Flag::E);
        insert!(jmp => "jeq", 48, ==, Flag::E);
        insert!(jmp => "jle", 49, !=, Flag::G);
        insert!(jmp => "jl",  50, ==, Flag::L);
        insert!(jmp => "jge", 51, !=, Flag::L);
        insert!(jmp => "jg",  52, ==, Flag::G);

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