mod header;
mod mbc;

pub use header::CartridgeHeader;
pub use mbc::Mbc;

use tracing::{error, info, trace};

use crate::common::types::{Address, Byte};

pub struct MbcDefault {
    rom: Vec<Byte>,
}

impl MbcDefault {
    pub fn new(rom: Vec<Byte>) -> Self {
        Self { rom }
    }
}

impl Mbc for MbcDefault {
    fn read_rom(&self, _address: Address) -> Byte {
        0xFF
    }
    fn write_rom(&mut self, _addr: Address, _value: Byte) {}
    fn read_ram(&self, _address: Address) -> Byte {
        0xFF
    }
    fn write_ram(&mut self, _address: Address, _value: Byte) {}
}

pub struct Cartridge {
    header: CartridgeHeader,
    mbc: Box<dyn Mbc>,
}

impl Cartridge {
    pub fn new(rom: Vec<Byte>) -> Self {
        let header: CartridgeHeader = CartridgeHeader::parse(&rom);
        let mbc: Box<dyn Mbc> = Box::new(MbcDefault::new(rom));

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
}
