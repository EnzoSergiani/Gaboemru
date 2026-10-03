mod header;
mod mbc;
mod rom_only;

pub use header::CartridgeHeader;
pub use mbc::Mbc;

use tracing::{error, info, trace};

use crate::{
    cartridge::rom_only::RomOnly,
    common::types::{Address, Byte},
};

pub struct Cartridge {
    header: CartridgeHeader,
    mbc: Box<dyn Mbc>,
}

impl Cartridge {
    pub fn new(rom: Vec<Byte>) -> Self {
        let header: CartridgeHeader = CartridgeHeader::parse(&rom);
        let mbc: Box<dyn Mbc> = match header.cartridge_type {
            header::CartridgeType::RomOnly => Box::new(RomOnly::new(rom)),
            _ => {
                error!("Unsupported cartridge type: {:?}", header.cartridge_type);
                panic!("Unsupported cartridge type: {:?}", header.cartridge_type);
            }
        };

        info!("initialisation");
        info!(
            "ROM selected: title: {} ; MBC: {:?}",
            header.title, header.cartridge_type
        );
        Self { header, mbc }
    }

    pub fn read(&self, address: Address) -> Byte {
        trace!("Reading from address: {:#06X}", address);
        match address {
            0x0000..=0x7FFF => self.mbc.read_rom(address),
            0xA000..=0xBFFF => self.mbc.read_ram(address),
            _ => {
                error!("Out-of-range read at address: {:#06X}", address);
                0xFF
            }
        }
    }

    pub fn write(&mut self, address: Address, value: Byte) {
        trace!("Writing {:#04X} from address: {:#06X}", value, address);
        match address {
            0x0000..=0x7FFF => self.mbc.write_rom(address, value),
            0xA000..=0xBFFF => self.mbc.write_ram(address - 0xA000, value),
            _ => {
                error!(
                    "Out-of-range write of {:#04X} at address: {:#06X}",
                    value, address
                );
            }
        }
    }

    pub fn get_header(&self) -> &CartridgeHeader {
        &self.header
    }
}

#[cfg(test)]
mod tests {
    use crate::common::test_helpers::build_rom;

    use super::*;

    #[test]
    fn getter_header() {
        let cartridge = Cartridge::new(build_rom("TETRIS", 0x00, 0x00, 0x00));
        assert_eq!(cartridge.get_header().title, "TETRIS");
    }
}
