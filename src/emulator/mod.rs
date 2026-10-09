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

    fn step(&mut self) -> Cycles {
        trace!("Step executed");
        self.cpu.step(&mut self.bus)
    }

    pub fn run(&mut self) {
        loop {
            self.step();
            self.handle_interrupts();
        }
    }

    fn handle_interrupts(&mut self) {
        let pending = self.bus.interrupt_enable() & self.bus.interrupt_flag() & 0x1F;

        if pending == 0 {
            return;
        }

        self.cpu.clear_halted();

        if !self.cpu.ime() {
            return;
        }

        for bit in 0..=4 {
            if self.is_interrupt_pending(bit) {
                self.cpu.set_ime(false);
                self.bus
                    .set_interrupt_flag(self.bus.interrupt_flag() & !(1 << bit));
                self.cpu
                    .dispatch_interrupt(&mut self.bus, 0x40 + bit as u16 * 8);
                break;
            }
        }
    }

    fn is_interrupt_pending(&self, bit: u8) -> bool {
        let mask = 1 << bit;
        (self.bus.interrupt_enable() & self.bus.interrupt_flag() & mask) != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::test_helpers::build_rom,
        cpu::registers::{Flags, Registers},
    };

    use super::*;

    fn gb_with_registers(pc: u16, sp: u16, ime: bool) -> GameBoy {
        let mut gb = GameBoy::new(build_rom("", 0x00, 0x00, 0x00));
        let mut registers = Registers {
            a: 0x01,
            f: Flags::from_byte(0xB0),
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
        };
        registers.pc = pc;
        registers.sp = sp;
        gb.cpu.set_state(registers, ime);
        gb
    }

    #[test]
    fn single_pending_interrupt_dispatches_to_correct_vector() {
        let mut gb = gb_with_registers(0x1234, 0xFFFE, true);
        gb.bus.set_interrupt_enable(0x01);
        gb.bus.set_interrupt_flag(0x01);
        gb.handle_interrupts();
        let (regs, ime) = gb.cpu.state();
        assert_eq!(regs.pc, 0x0040);
        assert_eq!(regs.sp, 0xFFFC);
        assert!(!ime);
        assert_eq!(gb.bus.interrupt_flag() & 0x01, 0);
        assert_eq!(gb.bus.read(0xFFFC), 0x34);
        assert_eq!(gb.bus.read(0xFFFD), 0x12);
    }

    #[test]
    fn multiple_pending_interrupts_services_lowest_bit_only() {
        let mut gb = gb_with_registers(0x1234, 0xFFFE, true);
        gb.bus.set_interrupt_enable(0x05);
        gb.bus.set_interrupt_flag(0x05);
        gb.handle_interrupts();
        let (regs, _) = gb.cpu.state();
        assert_eq!(regs.pc, 0x0040);
        assert_eq!(gb.bus.interrupt_flag() & 0x01, 0);
        assert_eq!(gb.bus.interrupt_flag() & 0x04, 0x04);
    }

    #[test]
    fn pending_interrupt_wakes_halted_cpu_without_dispatch_when_ime_false() {
        let mut rom = build_rom("", 0x00, 0x00, 0x00);
        rom[0x0100] = 0x76;
        let mut gb = GameBoy::new(rom);
        gb.cpu.set_ime(false);
        gb.step();
        assert!(gb.cpu.is_halted());
        gb.bus.set_interrupt_enable(0x01);
        gb.bus.set_interrupt_flag(0x01);
        let (regs_before, _) = gb.cpu.state();
        let pc_before = regs_before.pc;
        gb.handle_interrupts();
        assert!(!gb.cpu.is_halted());
        let (regs_after, _) = gb.cpu.state();
        assert_eq!(regs_after.pc, pc_before);
    }
}
