use tracing::{info, trace, warn};

use crate::common::types::{Address, Byte};

pub struct Bus {
}

impl Bus {
    pub fn new(rom: Vec<Byte>) -> Self {
        info!("initialisation");
        Self {}

    pub fn read(&self, address: Address) -> Byte {
        trace!("Reading from address: {:#06X}", address);
        match address {
            _ => {
                warn!("Out-of-range read at address: {:#06x}", address);
                0xFF
            }
        }
    }

    pub fn write(&mut self, address: Address, value: Byte) {
        trace!("Writing {:#04X} to address: {:#06X}", value, address);
        match address {
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
}
