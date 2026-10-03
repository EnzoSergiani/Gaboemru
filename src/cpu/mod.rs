mod registers;

use tracing::info;

use crate::cpu::registers::Registers;

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        info!("initialisation");
        Self {
            registers: Registers::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_post_boot_state() {
        let cpu = Cpu::new();
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.registers.f.to_byte(), 0xB0);
        assert_eq!(cpu.registers.bc(), 0x0013);
        assert_eq!(cpu.registers.de(), 0x00D8);
        assert_eq!(cpu.registers.hl(), 0x014D);
        assert_eq!(cpu.registers.sp, 0xFFFE);
        assert_eq!(cpu.registers.pc, 0x0100);
    }
}
