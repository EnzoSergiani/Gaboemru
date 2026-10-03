mod decoder;
mod registers;

use tracing::{info, trace};

use crate::{
    common::{
        bus::Bus,
        types::{Byte, Word},
    },
    cpu::registers::Registers,
};

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        info!("initialisation");
        Self {
            registers: Registers::default(),
        }
    }

    fn fetch_byte<B: Bus>(&mut self, bus: &mut B) -> Byte {
        let byte = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        trace!(
            "Fetched Program Counter: {:#04X}, next Program Counter: {:#04X}",
            byte, self.registers.pc
        );
        byte
    }

    fn fetch_word<B: Bus>(&mut self, bus: &mut B) -> Word {
        let lo = self.fetch_byte(bus) as Word;
        let hi = self.fetch_byte(bus) as Word;
        let word = (hi << 8) | lo;
        trace!(
            "Fetched word: {:#06X} (PC now at {:#06X})",
            word, self.registers.pc
        );
        word
    }
}

#[cfg(test)]
mod tests {
    use crate::common::test_helpers::FlatRam;

    use super::*;

    #[test]
    fn new_sets_post_boot_state() {
        let cpu = Cpu::new();
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.registers.f.to_byte(), 0xB0);
        assert_eq!(cpu.registers.bc(), 0x0013);
        assert_eq!(cpu.registers.de(), 0x00D8);
        assert_eq!(cpu.registers.hl(), 0x014D);
        assert_eq!(cpu.registers.sp, 0xFFFE);
        assert_eq!(cpu.registers.pc, 0x0100);
    }

    #[test]
    fn fetch_byte_advances_pc() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        bus.write(0x0100, 0x42);

        let pc_before = cpu.registers.pc;
        let byte = cpu.fetch_byte(&mut bus);

        assert_eq!(byte, 0x42);
        assert_eq!(cpu.registers.pc, pc_before.wrapping_add(1));
    }

    #[test]
    fn fetch_word_reads_little_endian() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        bus.write(0x0100, 0xCD);
        bus.write(0x0101, 0xAB);

        let word = cpu.fetch_word(&mut bus);
        assert_eq!(word, 0xABCD);
        assert_eq!(cpu.registers.pc, 0x0102);
    }

    #[test]
    fn fetch_wraps_at_0xffff() {
        let mut cpu = Cpu::new();
        cpu.registers.pc = 0xFFFF;
        let mut bus = FlatRam::new();
        bus.write(0xFFFF, 0x10);

        cpu.fetch_byte(&mut bus);
        assert_eq!(cpu.registers.pc, 0x0000);
    }
}
