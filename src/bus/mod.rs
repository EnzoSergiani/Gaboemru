use tracing::info;

use crate::common::types::Byte;

pub struct Bus {
}

impl Bus {
    pub fn new(rom: Vec<Byte>) -> Self {
        info!("initialisation");
        Self {}
    }
}
