use crate::common::types::{Byte, Cycles};

pub struct Serial {
    sb: Byte,
    sc: Byte,
    cycles_remaining: Cycles,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0x00,
            sc: 0x00,
            cycles_remaining: 0,
        }
    }

    pub fn sb(&self) -> Byte {
        self.sb
    }

    pub fn set_sb(&mut self, value: Byte) {
        self.sb = value;
    }

    pub fn sc(&self) -> Byte {
        self.sc | 0x7E
    }

    pub fn set_sc(&mut self, value: Byte) {
        self.sc = value & 0x81;
        if self.sc & 0x81 == 0x81 {
            self.cycles_remaining = 4096;
        }
    }
}

impl Default for Serial {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sb_roundtrips_through_setter() {
        let mut serial = Serial::new();
        serial.set_sb(0x42);
        assert_eq!(serial.sb(), 0x42);
    }

    #[test]
    fn sc_forces_unused_bits_to_one() {
        let mut serial = Serial::new();
        serial.set_sc(0x00);
        assert_eq!(serial.sc(), 0x7E);
    }
}
