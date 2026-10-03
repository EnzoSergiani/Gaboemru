use tracing::{error, info, trace};

use crate::common::types::{Address, Byte};

pub struct Cartridge {
}

impl Cartridge {
    pub fn new(rom: Vec<Byte>) -> Self {
        info!("initialisation");
        Self {}
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
