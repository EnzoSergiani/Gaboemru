use crate::common::types::{Byte, Word};

pub struct Timer {
    ticks: Word,
    tima: Byte,
}

impl Timer {
    pub fn new() -> Self {
        Self { ticks: 0, tima: 0 }
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
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
