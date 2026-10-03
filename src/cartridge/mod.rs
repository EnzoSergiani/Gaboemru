mod header;

pub use header::CartridgeHeader;

use tracing::{error, info, trace};

use crate::common::types::{Address, Byte};

pub struct Cartridge {
    header: CartridgeHeader,
}

impl Cartridge {
    pub fn new(rom: Vec<Byte>) -> Self {
        let header: CartridgeHeader = CartridgeHeader::parse(&rom);

        info!("initialisation");
        info!(
            "ROM selected: title: {} ; MBC: {:?}",
            header.title, header.cartridge_type
        );
        Self { header }
    }

    pub fn read(&self, address: Address) -> Byte {
        trace!("Reading from address: {:#06X}", address);
        match address {
            _ => {
                error!("Out-of-range read at address: {:#06X}", address);
                0xFF
            }
        }
    }

    pub fn write(&mut self, address: Address, value: Byte) {
        trace!("Writing {:#04X} from address: {:#06X}", value, address);
        match address {
            _ => {
                error!(
                    "Out-of-range write of {:#04X} at address: {:#06X}",
                    value, address
                );
            }
        }
    }
}
