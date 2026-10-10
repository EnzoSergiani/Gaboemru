use tracing::{debug, error, info, trace};

use crate::{
    cartridge::Cartridge,
    common::{
        bus::Bus as BusTrait,
        types::{Address, Byte, Cycles},
    },
    serial::Serial,
    timer::Timer,
};

impl BusTrait for Bus {
    fn read(&self, address: Address) -> Byte {
        self.read(address)
    }
    fn write(&mut self, address: Address, value: Byte) {
        self.write(address, value)
    }
}

pub struct Bus {
    cartridge: Cartridge,
    wram: [Byte; 0x2000],
    hram: [Byte; 0x7F],
    interrupt_enable: Byte,
    interrupt_flag: Byte,
    timer: Timer,
    serial: Serial,
}

impl Bus {
    pub fn new(rom: Vec<Byte>) -> Self {
        info!("initialisation");
        Self {
            cartridge: Cartridge::new(rom),
            wram: [0xFF; 0x2000],
            hram: [0xFF; 0x7F],
            interrupt_enable: 0x00,
            interrupt_flag: 0x00,
            serial: Serial::new(),
            timer: Timer::new(),
        }
    }

    pub fn read(&self, address: Address) -> Byte {
        trace!("Reading from address: {:#06X}", address);
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.read(address),
            0x8000..=0x9FFF => {
                debug!(
                    "VRAM not yet implemented, read at address: {:#06x}",
                    address
                );
                0xFF
            }
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => {
                debug!(
                    "OAM not yet implemented, range read at address: {:#06x}",
                    address
                );
                0xFF
            }
            0xFEA0..=0xFEFF => {
                error!("Prohibited range read at address: {:#06x}", address);
                0xFF
            }
            0xFF00 => {
                debug!(
                    "I/O registers not yet implemented, read at address: {:#06x}",
                    address
                );
                0xFF
            }
            0xFF01 => self.serial.sb(),
            0xFF02 => self.serial.sc(),
            0xFF03 => {
                debug!(
                    "I/O registers not yet implemented, read at address: {:#06x}",
                    address
                );
                0xFF
            }

            0xFF04 => self.timer.div(),
            0xFF05 => self.timer.tima(),
            0xFF06 => self.timer.tma(),
            0xFF07 => self.timer.tac(),
            0xFF08..=0xFF0E => {
                debug!(
                    "I/O registers not yet implemented, read at address: {:#06x}",
                    address
                );
                0xFF
            }

            0xFF0F => self.interrupt_flag | 0xE0,
            0xFF10..=0xFF7F => {
                debug!(
                    "I/O registers not yet implemented, read at address: {:#06x}",
                    address
                );
                0xFF
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable | 0xE0,
        }
    }

    pub fn write(&mut self, address: Address, value: Byte) {
        trace!("Writing {:#04X} to address: {:#06X}", value, address);
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => {
                self.cartridge.write(address, value);
            }
            0x8000..=0x9FFF => {
                debug!(
                    "VRAM not yet implemented, write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => {
                debug!(
                    "OAM not yet implemented, write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
            0xFEA0..=0xFEFF => {
                error!(
                    "Prohibited range write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
            0xFF00 => {
                debug!(
                    "I/O registers not yet implemented, write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
            0xFF01 => self.serial.set_sb(value),
            0xFF02 => self.serial.set_sc(value),
            0xFF03 => {
                debug!(
                    "I/O registers not yet implemented, write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }

            0xFF04 => self.timer.reset_div(),
            0xFF05 => self.timer.set_tima(value),
            0xFF06 => self.timer.set_tma(value),
            0xFF07 => self.timer.set_tac(value),
            0xFF08..=0xFF0E => {
                debug!(
                    "I/O registers not yet implemented, write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }

            0xFF0F => self.interrupt_flag = value & 0x1F,
            0xFF10..=0xFF7F => {
                debug!(
                    "I/O registers not yet implemented, write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt_enable = value & 0x1F,
        }
    }

    pub fn interrupt_enable(&self) -> Byte {
        self.interrupt_enable | 0xE0
    }

    pub fn set_interrupt_enable(&mut self, value: Byte) {
        self.interrupt_enable = value & 0x1F;
    }

    pub fn interrupt_flag(&self) -> Byte {
        self.interrupt_flag | 0xE0
    }

    pub fn set_interrupt_flag(&mut self, value: Byte) {
        self.interrupt_flag = value & 0x1F;
    }

    pub fn tick(&mut self, cycles: Cycles) {
        self.timer.tick(cycles);
        if self.timer.is_interrupt_requested() {
            self.interrupt_flag |= 0x04;
        }
        self.serial.tick(cycles);
        if self.serial.is_interrupt_requested() {
            self.interrupt_flag |= 0x08;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::test_helpers::build_rom;

    use super::*;

    #[test]
    fn reads_and_writes_wram() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xC000, 0xA0);
        assert_eq!(bus.read(0xC000), 0xA0);
    }

    #[test]
    fn reads_and_writes_hram() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xFF80, 0xA0);
        assert_eq!(bus.read(0xFF80), 0xA0);
    }

    #[test]
    fn prohibited_range_reads() {
        let bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        assert_eq!(bus.read(0xFEB0), 0xFF);
    }

    #[test]
    fn reads_cartridge_rom() {
        let mut rom = build_rom("", 0x00, 0x00, 0x00);
        rom[0x0000] = 0xAB;
        let bus = Bus::new(rom);
        assert_eq!(bus.read(0x0000), 0xAB);
    }

    #[test]
    fn reads_cartridge_ram() {
        let bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        assert_eq!(bus.read(0xA000), 0xFF);
    }

    #[test]
    fn getter_cartridge_header() {
        let bus = Bus::new(build_rom("TETRIS", 0x00, 0x00, 0x00));
        assert_eq!(bus.cartridge.get_header().title, "TETRIS");
    }

    #[test]
    fn reads_and_writes_timer_registers() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xFF06, 0x42);
        assert_eq!(bus.read(0xFF06), 0x42);
        bus.write(0xFF07, 0b101);
        assert_eq!(bus.read(0xFF07), 0b1111_1101);
        bus.write(0xFF05, 0x10); // TIMA
        assert_eq!(bus.read(0xFF05), 0x10);
    }

    #[test]
    fn writing_div_resets_it_regardless_of_value() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.tick(1000);
        assert_ne!(bus.read(0xFF04), 0x00);
        bus.write(0xFF04, 0xAB);
        assert_eq!(bus.read(0xFF04), 0x00);
    }

    #[test]
    fn tick_requests_timer_interrupt_on_tima_overflow() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xFF07, 0b101);
        bus.write(0xFF05, 0xFF);
        bus.tick(16);
        assert_eq!(bus.interrupt_flag() & 0x04, 0x04);
    }

    #[test]
    fn interrupt_enable_masks_unused_bits() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xFFFF, 0xFF);
        assert_eq!(bus.read(0xFFFF), 0xFF);
    }

    #[test]
    fn interrupt_flag_masks_unused_bits() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xFF0F, 0xFF);
        assert_eq!(bus.read(0xFF0F), 0xFF);
    }

    #[test]
    fn reads_and_writes_serial_registers() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xFF01, 0x42);
        assert_eq!(bus.read(0xFF01), 0x42);
        bus.write(0xFF02, 0x00);
        assert_eq!(bus.read(0xFF02), 0x7E);
    }

    #[test]
    fn tick_completes_serial_transfer_and_requests_interrupt() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xFF02, 0x81);
        bus.tick(4096);
        assert_eq!(bus.read(0xFF02) & 0x80, 0x00);
        assert_eq!(bus.read(0xFF01), 0xFF);
        assert_eq!(bus.interrupt_flag() & 0x08, 0x08);
    }

    #[test]
    fn tick_does_not_complete_serial_transfer_before_4096_cycles() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xFF02, 0x81);
        bus.tick(4095);
        assert_eq!(bus.read(0xFF02) & 0x80, 0x80);
        assert_eq!(bus.interrupt_flag() & 0x08, 0x00);
    }

    #[test]
    fn external_clock_serial_transfer_stays_pending() {
        let mut bus = Bus::new(build_rom("", 0x00, 0x00, 0x00));
        bus.write(0xFF02, 0x80);
        bus.tick(100_000);
        assert_eq!(bus.read(0xFF02) & 0x80, 0x80);
        assert_eq!(bus.interrupt_flag() & 0x08, 0x00);
    }
}
