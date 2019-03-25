use std::collections::HashMap;
use std::vec::Vec;

pub type Word = u32;

#[derive(Clone)]
pub enum CmdFormat { RM, RR, RI }

pub type CmdTable = HashMap<&'static str, (u8, CmdFormat)>;

pub fn init_cmd_table() -> CmdTable {
    let mut table = HashMap::new();
    macro_rules! defcmd {
        ($name:expr, $num:expr, $format:expr) => {
            table.insert(stringify!($name), ($num, $format))
        }
    }
    defcmd!(halt, 0, CmdFormat::RI);
    defcmd!(syscall, 1, CmdFormat::RI);

    table
}

pub struct CpuState {
    pub mem: Vec<Word>,
    pub r: [Word; 16],
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState{ mem: vec![0; 1 << 20], r: [0; 16] }
    }
}
