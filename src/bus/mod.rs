use tracing::{info, trace, warn};

use crate::common::types::{Address, Byte};

pub struct Bus {
    wram: [Byte; 0x2000],
    hram: [Byte; 0x7F],
}

impl Bus {
    pub fn new(rom: Vec<Byte>) -> Self {
        info!("initialisation");
        Self {
            wram: [0xFF; 0x2000],
            hram: [0xFF; 0x7F],
        }
    }

    pub fn read(&self, address: Address) -> Byte {
        trace!("Reading from address: {:#06X}", address);
        match address {
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => {
                warn!("Prohibited range read at address: {:#06x}", address);
                0xFF
            }
            0xFEA0..=0xFEFF => {
                warn!("Prohibited range read at address: {:#06x}", address);
                0xFF
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            _ => {
                warn!("Out-of-range read at address: {:#06x}", address);
                0xFF
            }
        }
    }

    pub fn write(&mut self, address: Address, value: Byte) {
        trace!("Writing {:#04X} to address: {:#06X}", value, address);
        match address {
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => {
                warn!(
                    "Prohibited range write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
            0xFEA0..=0xFEFF => {
                warn!(
                    "Prohibited range write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            _ => {
                warn!(
                    "Out-of-range write of {:#04X} at address: {:#06x}",
                    value, address
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_rom() -> Vec<Byte> {
        let mut rom = vec![0u8; 0x150];
        rom[0x0147] = 0x00;
        let checksum =
            (0x0134..=0x014C).fold(0u8, |acc, addr| acc.wrapping_sub(rom[addr]).wrapping_sub(1));
        rom[0x014D] = checksum;
        rom
    }

    #[test]
    fn reads_and_writes_wram() {
        let mut bus = Bus::new(minimal_rom());
        bus.write(0xC000, 0xA0);
        assert_eq!(bus.read(0xC000), 0xA0);
    }

    #[test]
    fn reads_and_writes_hram() {
        let mut bus = Bus::new(minimal_rom());
        bus.write(0xFF80, 0xA0);
        assert_eq!(bus.read(0xFF80), 0xA0);
    }

    #[test]
    fn prohibited_range_reads() {
        let bus = Bus::new(minimal_rom());
        assert_eq!(bus.read(0xFEB0), 0xFF);
    }
}
