use crate::common::types::{Byte, Cycles};

pub struct Serial {
    sb: Byte,
    sc: Byte,
    cycles_remaining: Cycles,
    interrupt_requested: bool,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0x00,
            sc: 0x00,
            cycles_remaining: 0,
            interrupt_requested: false,
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

    pub fn is_interrupt_requested(&mut self) -> bool {
        let requested = self.interrupt_requested;
        self.interrupt_requested = false;
        requested
    }

    pub fn tick(&mut self, cycles: Cycles) {
        if self.cycles_remaining == 0 {
            return;
        }
        self.cycles_remaining = self.cycles_remaining.saturating_sub(cycles);
        if self.cycles_remaining == 0 {
            self.sb = 0xFF;
            self.sc &= !0x80;
            self.interrupt_requested = true;
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

    #[test]
    fn no_transfer_started_tick_does_nothing() {
        let mut serial = Serial::new();
        serial.tick(10_000);
        assert_eq!(serial.sb(), 0x00);
        assert_eq!(serial.sc() & 0x80, 0x00);
    }

    #[test]
    fn internal_clock_transfer_completes_after_4096_cycles() {
        let mut serial = Serial::new();
        serial.set_sc(0x81);
        serial.tick(4095);
        assert_eq!(serial.sc() & 0x80, 0x80);
        assert_eq!(serial.sb(), 0x00);
        serial.tick(1);
        assert_eq!(serial.sc() & 0x80, 0x00);
        assert_eq!(serial.sb(), 0xFF);
    }

    #[test]
    fn internal_clock_transfer_requests_interrupt_on_completion() {
        let mut serial = Serial::new();
        serial.set_sc(0x81);
        serial.tick(4096);
        assert!(serial.is_interrupt_requested());
    }

    #[test]
    fn is_interrupt_requested_clears_after_read() {
        let mut serial = Serial::new();
        serial.set_sc(0x81);
        serial.tick(4096);
        assert!(serial.is_interrupt_requested());
        assert!(!serial.is_interrupt_requested());
    }

    #[test]
    fn external_clock_transfer_never_completes_without_a_peer() {
        let mut serial = Serial::new();
        serial.set_sc(0x80);
        serial.tick(1_000_000);
        assert_eq!(serial.sc() & 0x80, 0x80);
        assert_eq!(serial.sb(), 0x00);
    }
}
