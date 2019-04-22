use super::cmdspec::*;

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
            panic!("Invalid amount of args ({}) for {}", toks.len(), toks[0]);
        })
    }

    match cmd_data.1 {
        CmdFormat::RM => {
            checkargs!(2);
            let reg : u32 = partok!(r => 1);
            let adr : u32 = partok!(m => 2);

            (cmd_data.0 as u32) + (reg << 8) + (adr << 12)
        },
        CmdFormat::RR => {
            checkargs!(3);
            let reg1 : u32 = partok!(r => 1);
            let reg2 : u32 = partok!(r => 2);
            let imm  : u32 = partok!(i => 3);

            (cmd_data.0 as u32) + (reg1 << 8) + (reg2 << 12) + (imm << 16)
        },
        CmdFormat::RI => {
            checkargs!(2);
            let reg : u32 = partok!(r => 1);
            let imm : u32 = partok!(i => 2);

            (cmd_data.0 as u32) + (reg << 8) + (imm << 12)
        },
        CmdFormat::JMEM => {
            checkargs!(1);
            let adr : u32 = partok!(m => 1);

            (cmd_data.0 as u32) + (adr << 12)
        }
    }
}

pub fn parsecode(code: &str) -> CPU {
    let mut cpu = CPU::new();
    let lines = code.lines();
    let mut labeltabel : HashMap<&str, u32> = HashMap::new();
    let mut labeled = false;

    while {
        let mut cmdnum : u32 = 0;
        for l in lines.clone() {
            if l.chars().all(char::is_whitespace) {
                continue;
            }

            if l.chars().last() == Some(':') {
                if !labeled {
                    let len = l.chars().count();
                    labeltabel.insert(&l[..len - 1], cmdnum);
                }
                continue;
            }

            if labeled && l.trim() != "word" {
                let toks : Vec<&str> = l.split_whitespace().collect();
                if toks[0] == "end" {
                    cpu.state.r[15] = *labeltabel.get(toks[1]).expect(&format!("Invalid label! {}", toks[1]));
                } else {
                    cpu.state.mem[cmdnum as usize] = makeword(toks, &cpu.table, &labeltabel);
                }
            }
            cmdnum += 1;
        }
        !labeled
    } { labeled = true }

    cpu.state.r[14] = (MEMSZ - 1) as Word;

    cpu
}