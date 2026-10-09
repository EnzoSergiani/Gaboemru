use crate::common::types::{Byte, Word};

pub struct Timer {
    ticks: Word,
}

impl Timer {
    pub fn new() -> Self {
        Self { ticks: 0 }
    }

    pub fn div(&self) -> Byte {
        (self.ticks >> 8) as Byte
    }

    pub fn reset_div(&mut self) {
        self.ticks = 0;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
