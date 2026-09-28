use tracing::trace;

use crate::common::types::Byte;

pub struct GameBoy {}

impl GameBoy {
    pub fn new(_rom: Vec<Byte>) -> Self {
        trace!("GameBoy created");
        Self {}
    }
}
