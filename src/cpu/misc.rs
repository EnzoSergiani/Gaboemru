use crate::common::types::{Byte, Cycles};

use super::Cpu;

pub fn nop(_cpu: &mut Cpu) -> Cycles {
    4
}

pub fn stop(cpu: &mut Cpu) -> Cycles {
    cpu.stopped = true;
    12
}

pub fn halt(cpu: &mut Cpu) -> Cycles {
    cpu.halted = true;
    4
}

pub fn di(cpu: &mut Cpu) -> Cycles {
    cpu.ime = false;
    4
}

pub fn ei(cpu: &mut Cpu) -> Cycles {
    cpu.ime_scheduled = true;
    4
}

pub fn rlca(cpu: &mut Cpu) -> Cycles {
    let carry: bool = cpu.registers.a & 0x80 != 0;
    cpu.registers.a = cpu.registers.a.rotate_left(1);

    cpu.registers.f.z = false;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = carry;
    4
}

pub fn rrca(cpu: &mut Cpu) -> Cycles {
    let carry: bool = cpu.registers.a & 0x01 != 0;
    cpu.registers.a = cpu.registers.a.rotate_right(1);

    cpu.registers.f.z = false;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = carry;
    4
}

pub fn rla(cpu: &mut Cpu) -> Cycles {
    let old_carry: Byte = cpu.registers.f.c as Byte;
    let new_carry: bool = cpu.registers.a & 0x80 != 0;
    cpu.registers.a = (cpu.registers.a << 1) | old_carry;

    cpu.registers.f.z = false;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = new_carry;
    4
}

pub fn rra(cpu: &mut Cpu) -> Cycles {
    let old_carry: Byte = cpu.registers.f.c as Byte;
    let new_carry: bool = cpu.registers.a & 0x01 != 0;
    cpu.registers.a = (cpu.registers.a >> 1) | (old_carry << 7);

    cpu.registers.f.z = false;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = new_carry;
    4
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::Cpu;

    #[test]
    fn nop_does_nothing_but_consumes_cycles() {
        let mut cpu = Cpu::new();
        let registers_before = cpu.registers.a;
        let cycles = nop(&mut cpu);
        assert_eq!(cpu.registers.a, registers_before);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn halt_sets_halted_flag() {
        let mut cpu = Cpu::new();
        let cycles = halt(&mut cpu);
        assert!(cpu.halted);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn stop_sets_stopped_flag_and_costs_12_cycles() {
        let mut cpu = Cpu::new();
        let pc_before = cpu.registers.pc;
        let cycles = stop(&mut cpu);
        assert!(cpu.stopped);
        assert_eq!(cpu.registers.pc, pc_before);
        assert_eq!(cycles, 12);
    }

    #[test]
    fn di_clears_ime_immediately() {
        let mut cpu = Cpu::new();
        cpu.ime = true;
        di(&mut cpu);
        assert!(!cpu.ime);
    }

    #[test]
    fn ei_schedules_but_does_not_set_ime_immediately() {
        let mut cpu = Cpu::new();
        cpu.ime = false;
        ei(&mut cpu);
        assert!(!cpu.ime);
        assert!(cpu.ime_scheduled);
    }

    #[test]
    fn rlca_rotates_and_sets_carry_from_bit7() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b1000_0001;
        rlca(&mut cpu);
        assert_eq!(cpu.registers.a, 0b0000_0011);
        assert!(cpu.registers.f.c);
        assert!(!cpu.registers.f.z);
    }

    #[test]
    fn rlca_never_sets_zero_flag_even_on_zero_result() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x00;
        rlca(&mut cpu);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(!cpu.registers.f.z);
    }

    #[test]
    fn rrca_rotates_and_sets_carry_from_bit0() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b0000_0001;
        rrca(&mut cpu);
        assert_eq!(cpu.registers.a, 0b1000_0000);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn rla_shifts_old_carry_into_bit0() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b0100_0000;
        cpu.registers.f.c = true;
        rla(&mut cpu);
        assert_eq!(cpu.registers.a, 0b1000_0001);
        assert!(!cpu.registers.f.c);
    }

    #[test]
    fn rra_shifts_old_carry_into_bit7() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b0000_0010;
        cpu.registers.f.c = true;
        rra(&mut cpu);
        assert_eq!(cpu.registers.a, 0b1000_0001);
        assert!(!cpu.registers.f.c);
    }
}
