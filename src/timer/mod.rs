use crate::common::types::{Byte, Cycles, Word};

pub struct Timer {
    div: Word,
    tima: Byte,
    tma: Byte,
    tac: Byte,
    interrupt_requested: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            interrupt_requested: false,
        }
    }

    pub fn div(&self) -> Byte {
        (self.div >> 8) as Byte
    }

    pub fn reset_div(&mut self) {
        self.div = 0;
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

    fn is_enabled(&self) -> bool {
        self.tac & 0x04 != 0
    }

    pub fn is_interrupt_requested(&mut self) -> bool {
        let requested = self.interrupt_requested;
        self.interrupt_requested = false;
        requested
    }

    fn frequency_bit(&self) -> Word {
        match self.tac & 0x03 {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, cycles: Cycles) {
        for _ in 0..cycles {
            let bit: Word = self.frequency_bit();
            let before: Word = (self.div >> bit) & 1;
            self.div = self.div.wrapping_add(1);
            let after: Word = (self.div >> bit) & 1;

            if self.is_enabled() && before == 1 && after == 0 {
                let (new_tima, is_overflowed) = self.tima.overflowing_add(1);
                if is_overflowed {
                    self.tima = self.tma;
                    self.interrupt_requested = true;
                } else {
                    self.tima = new_tima;
                }
            }
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::timer::Timer;

    #[test]
    fn div_write() {
        let mut timer = Timer::new();
        timer.div = 0xFF00;
        timer.reset_div();
        assert_eq!(timer.div(), 0x00);
    }

    #[test]
    fn tima_overflow() {
        let mut timer = Timer::new();
        timer.tima = 0xFF;
        let (_, is_overflowed) = timer.tima.overflowing_add(1);
        assert!(is_overflowed);
    }

    #[test]
    fn tac_clock_selection() {
        let mut timer = Timer::new();
        timer.tac = 0b0000_0000;
        assert_eq!(timer.frequency_bit(), 9);
        timer.tac = 0b0000_0001;
        assert_eq!(timer.frequency_bit(), 3);
        timer.tac = 0b0000_0010;
        assert_eq!(timer.frequency_bit(), 5);
        timer.tac = 0b0000_0011;
        assert_eq!(timer.frequency_bit(), 7);
    }

    #[test]
    fn tick_increments_tima_at_correct_frequency() {
        let mut timer = Timer::new();
        timer.set_tac(0b101);
        timer.tick(15);
        assert_eq!(timer.tima(), 0);
        timer.tick(1);
        assert_eq!(timer.tima(), 1);
    }

    #[test]
    fn tick_does_not_increment_tima_when_disabled() {
        let mut timer = Timer::new();
        timer.set_tac(0b001);
        timer.tick(100);
        assert_eq!(timer.tima(), 0);
    }

    #[test]
    fn tima_overflow_reloads_from_tma_and_requests_interrupt() {
        let mut timer = Timer::new();
        timer.set_tac(0b101);
        timer.set_tma(0x42);
        timer.tima = 0xFF;
        timer.tick(16);
        assert_eq!(timer.tima(), 0x42);
        assert!(timer.is_interrupt_requested());
    }

    #[test]
    fn is_interrupt_requested_clears_after_read() {
        let mut timer = Timer::new();
        timer.set_tac(0b101);
        timer.tima = 0xFF;
        timer.tick(16);
        assert!(timer.is_interrupt_requested());
        assert!(!timer.is_interrupt_requested());
    }

    #[test]
    fn div_increments_every_256_cycles() {
        let mut timer = Timer::new();
        timer.tick(255);
        assert_eq!(timer.div(), 0x00);
        timer.tick(1);
        assert_eq!(timer.div(), 0x01);
    }

    #[test]
    fn reset_div_zeroes_internal_counter() {
        let mut timer = Timer::new();
        timer.tick(1000);
        assert_ne!(timer.div(), 0x00);
        timer.reset_div();
        assert_eq!(timer.div(), 0x00);
    }
}
