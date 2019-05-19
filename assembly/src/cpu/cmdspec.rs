#[macro_use]
use super::*;
use std::io;

use CmdFormat::*;

impl CmdTable {
    pub fn new() -> CmdTable {
        let mut table = CmdTable {
            code: HashMap::new(),
            func: HashMap::new(),
            name: HashMap::new()
        };

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

        macro_rules! parseline {
            ($T:ty) => {{
                let mut in_txt = String::new();
                io::stdin().read_line(&mut in_txt)
                    .expect("Failed to read from stdin");
                let in_num = in_txt.trim().parse::<$T>()
                    .expect(&format!("STDin: expected {}, got {}", stringify!($T), in_txt));

                in_num
            }};
        }

        table.insert("halt", 0, RI, &|cpu, _| cpu.halt = true);
        table.insert("syscall", 1, RI, &|cpu, arg| {
            let (reg, imm) = prs!(RI => arg);

            match imm {
                0 => cpu.halt = true,
                100 => cpu.r[reg] = parseline!(u32),
                101 => cpu.writed(parseline!(f64), reg),
                102 => print!("{}", cpu.r[reg]),
                103 => print!("{}", cpu.scand(reg)),
                104 => cpu.r[reg] = parseline!(char) as u32,
                105 => print!("{}", (cpu.r[reg] as u8) as char),

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

        table.insert("calli", 41, JMEM, &|cpu, arg| {
            let adr = prs!(JM => arg);

            cpu.push(cpu.r[15] + 1);
            cpu.jump(adr);
        });

        table.insert("ret", 42, JMEM, &|cpu, arg| {
            let lay = prs!(JM => arg);

            let ret_adr = cpu.pop();
            for _ in 0..lay {
                cpu.pop();
            }

            cpu.jump(ret_adr);
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

        table.insert("load", 64, RM, &|cpu, arg| {
            let (reg, mem) = prs!(RM => arg);
            cpu.load(mem, reg);
        });

        table.insert("store", 65, RM, &|cpu, arg| {
            let (reg, mem) = prs!(RM => arg);
            cpu.store(reg, mem);
        });

        table.insert("load2", 66, RM, &|cpu, arg| {
            let (reg, mem) = prs!(RM => arg);
            cpu.load(mem, reg);
            cpu.load(mem + 1, reg + 1);
        });

        table.insert("store2", 67, RM, &|cpu, arg| {
            let (reg, mem) = prs!(RM => arg);
            cpu.store(reg, mem);
            cpu.store(reg + 1, mem + 1);
        });

        table.insert("loadr", 68, RR, &|cpu, arg| {
            let (r1, r2, imm) = prs!(RR => arg);
            cpu.load(cpu.r[r2] + imm, r1);
        });

        table.insert("storer", 69, RR, &|cpu, arg| {
            let (r1, r2, imm) = prs!(RR => arg);
            cpu.store(r1, cpu.r[r2] + imm);
        });

        table.insert("loadr2", 70, RR, &|cpu, arg| {
            let (r1, r2, imm) = prs!(RR => arg);
            cpu.load(cpu.r[r2] + imm, r1);
            cpu.load(cpu.r[r2] + imm + 1, r1 + 1);
        });

        table.insert("storer2", 71, RR, &|cpu, arg| {
            let (r1, r2, imm) = prs!(RR => arg);
            cpu.store(r1, cpu.r[r2] + imm);
            cpu.store(r1 + 1, cpu.r[r2] + imm + 1);
        });

        table.insert("$STACK", 255, JMEM, &|cpu, arg| {
            let num = prs!(JM => arg);

            println!("STACK DUMP:");
            for i in 0..num {
                print!("[{}] => {:?}", i, cpu.mem[MEMSZ - (i + 1) as usize]);
                if MEMSZ as u32 - i - 1 == cpu.r[14] { print!("*"); }
                print!("\n");
            }
        });
        table
    }
}