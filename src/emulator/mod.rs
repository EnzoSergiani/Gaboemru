use tracing::{info, trace};

use crate::{
    bus::Bus,
    common::types::{Byte, Cycles},
    cpu::Cpu,
};

pub struct GameBoy {
    bus: Bus,
    cpu: Cpu,
}

impl GameBoy {
    pub fn new(rom: Vec<Byte>) -> Self {
        let bus: Bus = Bus::new(rom);
        let cpu: Cpu = Cpu::new();
        info!("initialisation");
        Self { bus, cpu }
    }

    pub fn step(&mut self) -> Cycles {
        trace!("Step executed");
        self.cpu.step(&mut self.bus)
    }
}
