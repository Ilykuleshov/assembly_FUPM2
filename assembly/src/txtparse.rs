use std::vec::Vec;
use super::cmdspec::*;

pub fn makeword(cmd: &str, cmd_table: CmdTable) -> Word {
    let toks : Vec<_> = cmd.split(" ").collect();

    let search_res = cmd_table.get(toks[0]);
    let cmd_data;

    match search_res {
        Some(x) => cmd_data = x,
        None    => panic!("undefined command")
    }

    match cmd_data.1 {
        CmdFormat::RM => {
            let reg : u32 = toks[1][1..].parse().expect("RM: Bad reg parameter");
            let adr : u32 = toks[2]     .parse().expect("RM: Bad mem parameter");

            (cmd_data.0 as u32) + (reg << 8) + (adr << 12)
        },
        CmdFormat::RR => {
            let reg1 : u32 = toks[1][1..].parse().expect("RR: Bad reg1 parameter");
            let reg2 : u32 = toks[2][1..].parse().expect("RR: Bad reg2 parameter");
            let imm  : u32 = toks[3].parse::<i16>().expect("RR: Bad imm parameter") as u32;

            (cmd_data.0 as u32) + (reg1 << 8) + (reg2 << 12) + (imm << 16)
        },
        CmdFormat::RI => {
            let reg : u32 = toks[1][1..].parse().expect("RI: Bad reg parameter");
            let imm : u32 = toks[2].parse::<i32>().expect("RI: Bad imm parameter") as u32;

            (cmd_data.0 as u32) + (reg << 8) + (imm << 12)
        }
    }
}
