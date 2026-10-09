use crate::common::types::{Address, Byte, Cycles, Word};

use super::{Bus, Cpu};

pub fn push_word<B: Bus>(cpu: &mut Cpu, bus: &mut B, value: Word) {
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    bus.write(cpu.registers.sp, (value >> 8) as Byte);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    bus.write(cpu.registers.sp, value as Byte);
}

fn pop_word<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Word {
    let low: Byte = bus.read(cpu.registers.sp);
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let high: Byte = bus.read(cpu.registers.sp);
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    (high as Word) << 8 | low as Word
}

pub fn jp_nn<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.fetch_word(bus);
    cpu.registers.pc = address;
    16
}

pub fn jp_hl(cpu: &mut Cpu) -> Cycles {
    cpu.registers.pc = cpu.registers.hl();
    4
}

macro_rules! jp_cc_nn {
    ($name:ident, $cond:expr) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let address: Address = cpu.fetch_word(bus);
            if $cond(cpu) {
                cpu.registers.pc = address;
                16
            } else {
                12
            }
        }
    };
}

jp_cc_nn!(jp_nz_nn, |cpu: &Cpu| !cpu.registers.f.z);
jp_cc_nn!(jp_z_nn, |cpu: &Cpu| cpu.registers.f.z);
jp_cc_nn!(jp_nc_nn, |cpu: &Cpu| !cpu.registers.f.c);
jp_cc_nn!(jp_c_nn, |cpu: &Cpu| cpu.registers.f.c);

pub fn jr_e8<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let offset = cpu.fetch_byte(bus) as i8 as i16;
    cpu.registers.pc = (cpu.registers.pc as i16).wrapping_add(offset) as Word;
    12
}

macro_rules! jr_cc_e8 {
    ($name:ident, $cond:expr) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let offset = cpu.fetch_byte(bus) as i8 as i16;
            if $cond(cpu) {
                cpu.registers.pc = (cpu.registers.pc as i16).wrapping_add(offset) as Word;
                12
            } else {
                8
            }
        }
    };
}

jr_cc_e8!(jr_nz_e8, |cpu: &Cpu| !cpu.registers.f.z);
jr_cc_e8!(jr_z_e8, |cpu: &Cpu| cpu.registers.f.z);
jr_cc_e8!(jr_nc_e8, |cpu: &Cpu| !cpu.registers.f.c);
jr_cc_e8!(jr_c_e8, |cpu: &Cpu| cpu.registers.f.c);

pub fn call_nn<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.fetch_word(bus);
    push_word(cpu, bus, cpu.registers.pc);
    cpu.registers.pc = address;
    24
}

macro_rules! call_cc_nn {
    ($name:ident, $cond:expr) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let address: Address = cpu.fetch_word(bus);
            if $cond(cpu) {
                push_word(cpu, bus, cpu.registers.pc);
                cpu.registers.pc = address;
                24
            } else {
                12
            }
        }
    };
}

call_cc_nn!(call_nz_nn, |cpu: &Cpu| !cpu.registers.f.z);
call_cc_nn!(call_z_nn, |cpu: &Cpu| cpu.registers.f.z);
call_cc_nn!(call_nc_nn, |cpu: &Cpu| !cpu.registers.f.c);
call_cc_nn!(call_c_nn, |cpu: &Cpu| cpu.registers.f.c);

pub fn ret<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    cpu.registers.pc = pop_word(cpu, bus);
    16
}

pub fn reti<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    cpu.registers.pc = pop_word(cpu, bus);
    cpu.ime = true;
    16
}

macro_rules! ret_cc {
    ($name:ident, $cond:expr) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            if $cond(cpu) {
                cpu.registers.pc = pop_word(cpu, bus);
                20
            } else {
                8
            }
        }
    };
}

ret_cc!(ret_nz, |cpu: &Cpu| !cpu.registers.f.z);
ret_cc!(ret_z, |cpu: &Cpu| cpu.registers.f.z);
ret_cc!(ret_nc, |cpu: &Cpu| !cpu.registers.f.c);
ret_cc!(ret_c, |cpu: &Cpu| cpu.registers.f.c);

macro_rules! rst {
    ($name:ident, $vector:expr) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            push_word(cpu, bus, cpu.registers.pc);
            cpu.registers.pc = $vector;
            16
        }
    };
}

rst!(rst_00, 0x0000);
rst!(rst_08, 0x0008);
rst!(rst_10, 0x0010);
rst!(rst_18, 0x0018);
rst!(rst_20, 0x0020);
rst!(rst_28, 0x0028);
rst!(rst_30, 0x0030);
rst!(rst_38, 0x0038);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_helpers::FlatRam;
    use crate::cpu::Cpu;

    #[test]
    fn jp_nn_jumps_to_address() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        bus.write(cpu.registers.pc, 0x00);
        bus.write(cpu.registers.pc + 1, 0x02);
        let cycles = jp_nn(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, 0x0200);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn jp_hl_uses_hl_directly_without_reading_memory() {
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1234);
        let cycles = jp_hl(&mut cpu);
        assert_eq!(cpu.registers.pc, 0x1234);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn jp_nz_nn_jumps_when_not_zero() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.f.z = false;
        bus.write(cpu.registers.pc, 0x00);
        bus.write(cpu.registers.pc + 1, 0x02);
        let cycles = jp_nz_nn(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, 0x0200);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn jp_nz_nn_does_not_jump_when_zero() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.f.z = true;
        let pc_before = cpu.registers.pc;
        bus.write(cpu.registers.pc, 0x00);
        bus.write(cpu.registers.pc + 1, 0x02);
        let cycles = jp_nz_nn(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, pc_before.wrapping_add(2));
        assert_eq!(cycles, 12);
    }

    #[test]
    fn jr_e8_positive_offset() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        let pc_before = cpu.registers.pc;
        bus.write(cpu.registers.pc, 0x05);
        jr_e8(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, pc_before.wrapping_add(1).wrapping_add(5));
    }

    #[test]
    fn jr_e8_negative_offset() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.pc = 0x0150;
        bus.write(cpu.registers.pc, 0xFB);
        jr_e8(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, 0x014C);
    }

    #[test]
    fn jr_z_e8_always_reads_offset_even_when_not_taken() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.f.z = false;
        let pc_before = cpu.registers.pc;
        bus.write(cpu.registers.pc, 0x10);
        let cycles = jr_z_e8(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, pc_before.wrapping_add(1));
        assert_eq!(cycles, 8);
    }

    #[test]
    fn call_nn_pushes_return_address_and_jumps() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        let sp_before = cpu.registers.sp;
        let pc_before = cpu.registers.pc;
        bus.write(cpu.registers.pc, 0x00);
        bus.write(cpu.registers.pc + 1, 0x02);
        let cycles = call_nn(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, 0x0200);
        assert_eq!(cpu.registers.sp, sp_before.wrapping_sub(2));
        assert_eq!(cycles, 24);
        let pushed_low = bus.read(cpu.registers.sp);
        let pushed_high = bus.read(cpu.registers.sp + 1);
        let pushed = (pushed_high as Word) << 8 | pushed_low as Word;
        assert_eq!(pushed, pc_before.wrapping_add(2));
    }

    #[test]
    fn call_nz_nn_does_not_push_when_condition_false() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.f.z = true;
        let sp_before = cpu.registers.sp;
        bus.write(cpu.registers.pc, 0x00);
        bus.write(cpu.registers.pc + 1, 0x02);
        let cycles = call_nz_nn(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.sp, sp_before);
        assert_eq!(cycles, 12);
    }

    #[test]
    fn call_then_ret_roundtrip() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        let pc_before = cpu.registers.pc;
        bus.write(cpu.registers.pc, 0x00);
        bus.write(cpu.registers.pc + 1, 0x02);
        call_nn(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, 0x0200);
        let cycles = ret(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, pc_before.wrapping_add(2));
        assert_eq!(cycles, 16);
    }

    #[test]
    fn reti_restores_pc_and_sets_ime() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.ime = false;
        push_word(&mut cpu, &mut bus, 0x0150);
        let cycles = reti(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, 0x0150);
        assert!(cpu.ime);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn ret_nz_costs_8_cycles_when_not_taken() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.f.z = true;
        let sp_before = cpu.registers.sp;
        let cycles = ret_nz(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.sp, sp_before);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn rst_38_pushes_pc_and_jumps_to_fixed_vector() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        let pc_before = cpu.registers.pc;
        let cycles = rst_38(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.pc, 0x0038);
        assert_eq!(cycles, 16);
        let pushed = pop_word(&mut cpu, &mut bus);
        assert_eq!(pushed, pc_before);
    }

    #[test]
    fn push_pop_word_roundtrip() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        push_word(&mut cpu, &mut bus, 0xABCD);
        let value = pop_word(&mut cpu, &mut bus);
        assert_eq!(value, 0xABCD);
    }
}
