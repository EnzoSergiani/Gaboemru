use tracing::info;

use crate::{bus::Bus, common::types::Byte, cpu::Cpu};

pub struct GameBoy {
    bus: Bus,
    cpu: Cpu,
}

impl GameBoy {
    pub fn new(rom: Vec<Byte>) -> Self {
        let bus: Bus = Bus::new(rom);
        let cpu: Cpu = Cpu::new();
        info!("Initialisation");
        Self { bus, cpu }
    }
}
