use tracing::info;

use crate::{bus::Bus, common::types::Byte};

pub struct GameBoy {
    bus: Bus,
}

impl GameBoy {
    pub fn new(rom: Vec<Byte>) -> Self {
        let bus: Bus = Bus::new(rom);

        info!("initialisation");
        Self { bus }
    }
}
