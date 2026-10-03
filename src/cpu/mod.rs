pub struct Cpu {}
use tracing::info;


impl Cpu {
    pub fn new() -> Self {
        info!("Initialisation");
        Self {
        }
    }
}
