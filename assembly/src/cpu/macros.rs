macro_rules! getcode {
    ($word:expr) => {
        ($word >> 24) as u8
    }
}

macro_rules! prs {
    (RM => $word:expr) => { (
        (($word >> 20) & 15u32) as usize, 
        ($word & ((1 << 20) - 1))
        ) };
    (RR => $word:expr) => { (
        (($word >> 20) & 15u32) as usize, 
        (($word >> 16) & 15u32) as usize, 
        ($word.clone() as i16) as u32
        ) };
    (RI => $word:expr) => { (
        (($word >> 20) & 15u32) as usize, 
        (((($word.clone() as i32).wrapping_add(1 << 19)) & ((1 << 20) - 1)) - (1 << 19)) as u32
        ) };
    (JM => $word:expr) => { {
        let (_, mem) : (_, u32) = prs!(RM => $word);
        mem
        } };
}