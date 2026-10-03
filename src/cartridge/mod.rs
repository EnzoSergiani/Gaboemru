use tracing::info;

use crate::common::types::Byte;

pub struct Cartridge {
}

impl Cartridge {
    pub fn new(rom: Vec<Byte>) -> Self {
        info!("initialisation");
        Self {}
    }
}
