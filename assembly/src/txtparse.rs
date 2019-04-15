use std::vec::Vec;
use super::cmdspec::*;

use std::collections::HashMap;

pub fn makeword(cmd: &str, cmd_table: &CmdTable, labeltabel: &HashMap<&str, u32>) -> Word {
    let toks : Vec<_> = cmd.split_whitespace().collect();
    let cmd_data = cmd_table.get_code(toks[0]);

    macro_rules! partok {
        (r => $n:expr) => (toks[$n][1..].parse().expect("Bad reg parameter!"));
        (i => $n:expr) => (toks[$n].parse::<i32>().expect("Bad imm parameter!") as u32);
        (m => $n:expr) => (match toks[$n].parse::<u32>() {
            Ok(n) => n,
            Err(_) => labeltabel.get(toks[$n]).expect(&format!("Invalid label! ({})", toks[$n])).clone()
        });
    }

    match cmd_data.1 {
        CmdFormat::RM => {
            let reg : u32 = partok!(r => 1);
            let adr : u32 = partok!(m => 2);

            (cmd_data.0 as u32) + (reg << 8) + (adr << 12)
        },
        CmdFormat::RR => {
            let reg1 : u32 = partok!(r => 1);
            let reg2 : u32 = partok!(r => 2);
            let imm  : u32 = partok!(i => 3);

            (cmd_data.0 as u32) + (reg1 << 8) + (reg2 << 12) + (imm << 16)
        },
        CmdFormat::RI => {
            let reg : u32 = partok!(r => 1);
            let imm : u32 = partok!(i => 2);

            (cmd_data.0 as u32) + (reg << 8) + (imm << 12)
        },
        CmdFormat::JMEM => {
            let adr : u32 = partok!(m => 1);

            (cmd_data.0 as u32) + (adr << 12)
        }
    }
}

pub fn parsecode(code: &std::string::String) -> CPU {
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

            if labeled {
                cpu.state.mem[cmdnum as usize] = makeword(l, &cpu.table, &labeltabel);
            }
            cmdnum += 1;
        }
        !labeled
    } { labeled = true; for (name, line) in labeltabel.iter() { println!("{} - {}", name, line); }}

    cpu.state.r[14] = MEMSZ as Word;

    cpu
}