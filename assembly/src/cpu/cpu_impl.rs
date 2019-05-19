use super::*;

macro_rules! convd {
    ($fsti:expr, $sndi:expr) => { {
        let num : u64 = $fsti as u64 + (($sndi as u64) << 32);
        f64::from_bits(num)
    } };
    ($float:expr) => { {
        let num : u64 = $float.to_bits();
        let fst : u32 = (num & 0b11111111111111111111111111111111) as u32;
        let snd : u32 = (num >> 32) as u32;

        (fst, snd)
    } };
}

impl CpuState {
    pub fn scand(&self, reg: usize) -> f64 {
        convd!(self.r[reg], self.r[reg + 1])
    }

    pub fn writed(&mut self, f: f64, reg: usize) {
        let (u1, u2) = convd!(f);
        self.r[reg] = u1;
        self.r[reg + 1] = u2;
    }

    pub fn push(&mut self, val: Word) {
        self.r[14] -= 1;
        self.mem[self.r[14] as usize] = val;
    }

    pub fn pop(&mut self) -> u32 {
        let ret = self.mem[self.r[14] as usize];
        self.r[14] += 1;
        ret
    }

    pub fn cmp<T:PartialOrd>(&mut self, val1: T, val2: T) {
        if val1 < val2 {
            self.f = Flag::L;
        } else
        if val1 > val2 {
            self.f = Flag::G; 
        } else {
            self.f = Flag::E;
        }
    }

    pub fn jump(&mut self, adr: u32) {
        self.r[15] = adr.wrapping_sub(1);
    }

    pub fn load(&mut self, adr: u32, reg: usize) {
        self.r[reg] = self.mem[adr as usize];
    }

    pub fn store(&mut self, reg: usize, adr: u32) {
        self.mem[adr as usize] = self.r[reg];
    }
}