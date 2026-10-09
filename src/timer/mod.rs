use crate::common::types::{Byte, Word};

pub struct Timer {
    ticks: Word,
    tima: Byte,
    tma: Byte,
    tac: Byte,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn div(&self) -> Byte {
        (self.ticks >> 8) as Byte
    }

    pub fn reset_div(&mut self) {
        self.ticks = 0;
    }

    pub fn tima(&self) -> Byte {
        self.tima
    }

    pub fn set_tima(&mut self, value: Byte) {
        self.tima = value;
    }

    pub fn tma(&self) -> Byte {
        self.tma
    }

    pub fn set_tma(&mut self, value: Byte) {
        self.tma = value;
    }

    pub fn tac(&self) -> Byte {
        self.tac | 0xF8
    }

    pub fn set_tac(&mut self, value: Byte) {
        self.tac = value & 0x07;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
