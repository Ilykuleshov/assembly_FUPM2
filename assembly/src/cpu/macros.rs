macro_rules! getcode {
    ($word:expr) => {
        ($word >> 24) as u8
    }
}

macro_rules! prs {
    (RM => $word:expr) => { (
        (($word >> 20) & 15u32) as usize, 
        ($word & ((2 << 20) - 1))
        ) };
    (RR => $word:expr) => { (
        (($word >> 20) & 15u32) as usize, 
        (($word >> 16) & 15u32) as usize, 
        ((($word.clone().wrapping_add(2 << 16)) & ((2 << 16) - 1)) as u32).wrapping_sub(2 << 16)
        ) };
    (RI => $word:expr) => { (
        (($word >> 20) & 15u32) as usize, 
        ((($word.clone().wrapping_add(2 << 19)) & ((2 << 20) - 1)) as u32).wrapping_sub(2 << 19)
        ) };
    (JM => $word:expr) => { {
        let (_, mem) : (_, u32) = prs!(RM => $word);
        mem
        } };
}