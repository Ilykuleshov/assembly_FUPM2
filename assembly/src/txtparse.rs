use super::cpu::*;

use std::collections::HashMap;
use std::vec::Vec;

pub fn makeword(toks: Vec<&str>, cmd_table: &CmdTable, labeltabel: &HashMap<&str, u32>) -> Word {
    let cmd_data = cmd_table.get_code(toks[0]);

    macro_rules! partok {
        (r => $n:expr) => (toks[$n][1..].parse().expect("Bad reg parameter!"));
        (i => $n:expr) => (toks[$n].parse::<i32>().expect("Bad imm parameter!") as u32);
        (m => $n:expr) => (match toks[$n].parse::<u32>() {
            Ok(n) => n,
            Err(_) => labeltabel.get(toks[$n]).expect(&format!("Invalid label! ({})", toks[$n])).clone()
        });
    }

    macro_rules! checkargs {
        ($num:expr) => (if toks.len() - 1 != $num {
            panic!("Invalid amount of args ({:?}) for {}", toks, toks[0]);
        })
    }

    match cmd_data.1 {
        CmdFormat::RM => {
            checkargs!(2);
            let reg : u32 = partok!(r => 1);
            let adr : u32 = partok!(m => 2);

            ((cmd_data.0 as u32) << 24) + (reg << 20) + adr
        },
        CmdFormat::RR => {
            checkargs!(3);
            let reg1 : u32 = partok!(r => 1);
            let reg2 : u32 = partok!(r => 2);
            let imm  : u32 = partok!(i => 3);

            ((cmd_data.0 as u32) << 24) + (reg1 << 20) + (reg2 << 16) + (imm & ((2 << 16) - 1))
        },
        CmdFormat::RI => {
            checkargs!(2);
            let reg : u32 = partok!(r => 1);
            let imm : u32 = partok!(i => 2);

            ((cmd_data.0 as u32) << 24) + (reg << 20) + (imm & ((2 << 20) - 1))
        },
        CmdFormat::JMEM => {
            checkargs!(1);
            let adr : u32 = partok!(m => 1);

            ((cmd_data.0 as u32) << 24) + adr
        }
    }
}

pub fn parsecode(code: &str) -> CPU {
    let mut cpu = CPU::new();
    let mut lines = code.lines();
    if code.starts_with('$') {
        let flags;
        match lines.next() {
            Some(x) => flags = x,
            _ => panic!("Smth went wrong!")
        }

        if flags.contains("CMD") { cpu.state.mode |= dbmode::CMD; }
        if flags.contains("ARG") { cpu.state.mode |= dbmode::ARG; }
        if flags.contains("REG") { cpu.state.mode |= dbmode::REG; }
    }
    let mut labeltabel : HashMap<&str, u32> = HashMap::new();
    let mut labeled = false;
    let mut cmdnum : u32 = 0;

    while {
        cmdnum = 0;
        for line in lines.clone() {
            //Remove comments
            let mut line = line.split_terminator(';').nth(0).unwrap().trim();

            //Check if label
            match line.find(':') {
                Some(size) =>  {
                    if !labeled {
                        labeltabel.insert(&line[0..size], cmdnum);
                    }

                    line = &line[size + 1..];
                }

                None => {}
            }

            if line.chars().all(char::is_whitespace) { continue; }

            if labeled && line != "word" {
                let toks : Vec<&str> = line.split_whitespace().collect();
                if toks[0] == "end" {
                    cpu.state.r[15] = *labeltabel.get(toks[1])
                                                 .expect(&format!("Invalid label! {}", toks[1]));
                } else {
                    cpu.state.mem[cmdnum as usize] = makeword(toks, &cpu.table, &labeltabel);
                }
            }
            cmdnum += 1;
        }
        !labeled
    } { labeled = true }

    cpu.state.progsz = cmdnum;
    cpu.state.r[14] = MEMSZ as Word;

    cpu
}