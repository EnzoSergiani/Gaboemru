use tracing::{debug, error, info, trace};

use crate::{
    cartridge::Cartridge,
    common::types::{Address, Byte},
};

pub struct Bus {
    cartridge: Cartridge,
    wram: [Byte; 0x2000],
    hram: [Byte; 0x7F],
}

impl Bus {
    pub fn new(rom: Vec<Byte>) -> Self {
        info!("initialisation");
        Self {
            cartridge: Cartridge::new(rom),
            wram: [0xFF; 0x2000],
            hram: [0xFF; 0x7F],
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
            0xFF00..=0xFF7F => {
                debug!(
                    "I/O registers not yet implemented, read at address: {:#06x}",
                    address
                );
                0xFF
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => {
                debug!(
                    "IE register not yet implemented, read at address: {:#06x}",
                    address
                );
                0xFF
            }
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
            0xFF00..=0xFF7F => {
                debug!(
                    "I/O registers not yet implemented, write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => {
                debug!(
                    "IE register not yet implemented, write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
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
}
