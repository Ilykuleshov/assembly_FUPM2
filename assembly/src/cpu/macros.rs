macro_rules! getcode {
    ($word:expr) => {
        ($word & 255) as u8
    }
}

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