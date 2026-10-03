use tracing::error;

use crate::{
    common::{
        bus::Bus,
        types::{Byte, Cycles},
    },
    cpu::Cpu,
};

pub fn execute<B: Bus>(cpu: &mut Cpu, bus: &mut B, opcode: Byte) -> Cycles {
    match opcode {
        _ => {
            error!("Opcode {:#04X} not yet implemented", opcode);
            0
        }
    }
}

