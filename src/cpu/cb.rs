use tracing::trace;

use crate::common::types::{Address, Byte, Cycles};

use super::{Bus, Cpu};

pub fn execute<B: Bus>(cpu: &mut Cpu, bus: &mut B, cb_opcode: Byte) -> Cycles {
    trace!("Execute prefix opcode: {:#04X}", cb_opcode);
    match cb_opcode {
        0x00 => rlc_b(cpu),
        0x01 => rlc_c(cpu),
        0x02 => rlc_d(cpu),
        0x03 => rlc_e(cpu),
        0x04 => rlc_h(cpu),
        0x05 => rlc_l(cpu),
        0x06 => rlc_hl(cpu, bus),
        0x07 => rlc_a(cpu),
        0x08 => rrc_b(cpu),
        0x09 => rrc_c(cpu),
        0x0A => rrc_d(cpu),
        0x0B => rrc_e(cpu),
        0x0C => rrc_h(cpu),
        0x0D => rrc_l(cpu),
        0x0E => rrc_hl(cpu, bus),
        0x0F => rrc_a(cpu),
        0x10 => rl_b(cpu),
        0x11 => rl_c(cpu),
        0x12 => rl_d(cpu),
        0x13 => rl_e(cpu),
        0x14 => rl_h(cpu),
        0x15 => rl_l(cpu),
        0x16 => rl_hl(cpu, bus),
        0x17 => rl_a(cpu),
        0x18 => rr_b(cpu),
        0x19 => rr_c(cpu),
        0x1A => rr_d(cpu),
        0x1B => rr_e(cpu),
        0x1C => rr_h(cpu),
        0x1D => rr_l(cpu),
        0x1E => rr_hl(cpu, bus),
        0x1F => rr_a(cpu),
        0x20 => sla_b(cpu),
        0x21 => sla_c(cpu),
        0x22 => sla_d(cpu),
        0x23 => sla_e(cpu),
        0x24 => sla_h(cpu),
        0x25 => sla_l(cpu),
        0x26 => sla_hl(cpu, bus),
        0x27 => sla_a(cpu),
        0x28 => sra_b(cpu),
        0x29 => sra_c(cpu),
        0x2A => sra_d(cpu),
        0x2B => sra_e(cpu),
        0x2C => sra_h(cpu),
        0x2D => sra_l(cpu),
        0x2E => sra_hl(cpu, bus),
        0x2F => sra_a(cpu),
        0x30 => swap_b(cpu),
        0x31 => swap_c(cpu),
        0x32 => swap_d(cpu),
        0x33 => swap_e(cpu),
        0x34 => swap_h(cpu),
        0x35 => swap_l(cpu),
        0x36 => swap_hl(cpu, bus),
        0x37 => swap_a(cpu),
        0x38 => srl_b(cpu),
        0x39 => srl_c(cpu),
        0x3A => srl_d(cpu),
        0x3B => srl_e(cpu),
        0x3C => srl_h(cpu),
        0x3D => srl_l(cpu),
        0x3E => srl_hl(cpu, bus),
        0x3F => srl_a(cpu),
        0x40 => bit_0_b(cpu),
        0x41 => bit_0_c(cpu),
        0x42 => bit_0_d(cpu),
        0x43 => bit_0_e(cpu),
        0x44 => bit_0_h(cpu),
        0x45 => bit_0_l(cpu),
        0x46 => bit_0_hl(cpu, bus),
        0x47 => bit_0_a(cpu),
        0x48 => bit_1_b(cpu),
        0x49 => bit_1_c(cpu),
        0x4A => bit_1_d(cpu),
        0x4B => bit_1_e(cpu),
        0x4C => bit_1_h(cpu),
        0x4D => bit_1_l(cpu),
        0x4E => bit_1_hl(cpu, bus),
        0x4F => bit_1_a(cpu),
        0x50 => bit_2_b(cpu),
        0x51 => bit_2_c(cpu),
        0x52 => bit_2_d(cpu),
        0x53 => bit_2_e(cpu),
        0x54 => bit_2_h(cpu),
        0x55 => bit_2_l(cpu),
        0x56 => bit_2_hl(cpu, bus),
        0x57 => bit_2_a(cpu),
        0x58 => bit_3_b(cpu),
        0x59 => bit_3_c(cpu),
        0x5A => bit_3_d(cpu),
        0x5B => bit_3_e(cpu),
        0x5C => bit_3_h(cpu),
        0x5D => bit_3_l(cpu),
        0x5E => bit_3_hl(cpu, bus),
        0x5F => bit_3_a(cpu),
        0x60 => bit_4_b(cpu),
        0x61 => bit_4_c(cpu),
        0x62 => bit_4_d(cpu),
        0x63 => bit_4_e(cpu),
        0x64 => bit_4_h(cpu),
        0x65 => bit_4_l(cpu),
        0x66 => bit_4_hl(cpu, bus),
        0x67 => bit_4_a(cpu),
        0x68 => bit_5_b(cpu),
        0x69 => bit_5_c(cpu),
        0x6A => bit_5_d(cpu),
        0x6B => bit_5_e(cpu),
        0x6C => bit_5_h(cpu),
        0x6D => bit_5_l(cpu),
        0x6E => bit_5_hl(cpu, bus),
        0x6F => bit_5_a(cpu),
        0x70 => bit_6_b(cpu),
        0x71 => bit_6_c(cpu),
        0x72 => bit_6_d(cpu),
        0x73 => bit_6_e(cpu),
        0x74 => bit_6_h(cpu),
        0x75 => bit_6_l(cpu),
        0x76 => bit_6_hl(cpu, bus),
        0x77 => bit_6_a(cpu),
        0x78 => bit_7_b(cpu),
        0x79 => bit_7_c(cpu),
        0x7A => bit_7_d(cpu),
        0x7B => bit_7_e(cpu),
        0x7C => bit_7_h(cpu),
        0x7D => bit_7_l(cpu),
        0x7E => bit_7_hl(cpu, bus),
        0x7F => bit_7_a(cpu),
        0x80 => res_0_b(cpu),
        0x81 => res_0_c(cpu),
        0x82 => res_0_d(cpu),
        0x83 => res_0_e(cpu),
        0x84 => res_0_h(cpu),
        0x85 => res_0_l(cpu),
        0x86 => res_0_hl(cpu, bus),
        0x87 => res_0_a(cpu),
        0x88 => res_1_b(cpu),
        0x89 => res_1_c(cpu),
        0x8A => res_1_d(cpu),
        0x8B => res_1_e(cpu),
        0x8C => res_1_h(cpu),
        0x8D => res_1_l(cpu),
        0x8E => res_1_hl(cpu, bus),
        0x8F => res_1_a(cpu),
        0x90 => res_2_b(cpu),
        0x91 => res_2_c(cpu),
        0x92 => res_2_d(cpu),
        0x93 => res_2_e(cpu),
        0x94 => res_2_h(cpu),
        0x95 => res_2_l(cpu),
        0x96 => res_2_hl(cpu, bus),
        0x97 => res_2_a(cpu),
        0x98 => res_3_b(cpu),
        0x99 => res_3_c(cpu),
        0x9A => res_3_d(cpu),
        0x9B => res_3_e(cpu),
        0x9C => res_3_h(cpu),
        0x9D => res_3_l(cpu),
        0x9E => res_3_hl(cpu, bus),
        0x9F => res_3_a(cpu),
        0xA0 => res_4_b(cpu),
        0xA1 => res_4_c(cpu),
        0xA2 => res_4_d(cpu),
        0xA3 => res_4_e(cpu),
        0xA4 => res_4_h(cpu),
        0xA5 => res_4_l(cpu),
        0xA6 => res_4_hl(cpu, bus),
        0xA7 => res_4_a(cpu),
        0xA8 => res_5_b(cpu),
        0xA9 => res_5_c(cpu),
        0xAA => res_5_d(cpu),
        0xAB => res_5_e(cpu),
        0xAC => res_5_h(cpu),
        0xAD => res_5_l(cpu),
        0xAE => res_5_hl(cpu, bus),
        0xAF => res_5_a(cpu),
        0xB0 => res_6_b(cpu),
        0xB1 => res_6_c(cpu),
        0xB2 => res_6_d(cpu),
        0xB3 => res_6_e(cpu),
        0xB4 => res_6_h(cpu),
        0xB5 => res_6_l(cpu),
        0xB6 => res_6_hl(cpu, bus),
        0xB7 => res_6_a(cpu),
        0xB8 => res_7_b(cpu),
        0xB9 => res_7_c(cpu),
        0xBA => res_7_d(cpu),
        0xBB => res_7_e(cpu),
        0xBC => res_7_h(cpu),
        0xBD => res_7_l(cpu),
        0xBE => res_7_hl(cpu, bus),
        0xBF => res_7_a(cpu),
        0xC0 => set_0_b(cpu),
        0xC1 => set_0_c(cpu),
        0xC2 => set_0_d(cpu),
        0xC3 => set_0_e(cpu),
        0xC4 => set_0_h(cpu),
        0xC5 => set_0_l(cpu),
        0xC6 => set_0_hl(cpu, bus),
        0xC7 => set_0_a(cpu),
        0xC8 => set_1_b(cpu),
        0xC9 => set_1_c(cpu),
        0xCA => set_1_d(cpu),
        0xCB => set_1_e(cpu),
        0xCC => set_1_h(cpu),
        0xCD => set_1_l(cpu),
        0xCE => set_1_hl(cpu, bus),
        0xCF => set_1_a(cpu),
        0xD0 => set_2_b(cpu),
        0xD1 => set_2_c(cpu),
        0xD2 => set_2_d(cpu),
        0xD3 => set_2_e(cpu),
        0xD4 => set_2_h(cpu),
        0xD5 => set_2_l(cpu),
        0xD6 => set_2_hl(cpu, bus),
        0xD7 => set_2_a(cpu),
        0xD8 => set_3_b(cpu),
        0xD9 => set_3_c(cpu),
        0xDA => set_3_d(cpu),
        0xDB => set_3_e(cpu),
        0xDC => set_3_h(cpu),
        0xDD => set_3_l(cpu),
        0xDE => set_3_hl(cpu, bus),
        0xDF => set_3_a(cpu),
        0xE0 => set_4_b(cpu),
        0xE1 => set_4_c(cpu),
        0xE2 => set_4_d(cpu),
        0xE3 => set_4_e(cpu),
        0xE4 => set_4_h(cpu),
        0xE5 => set_4_l(cpu),
        0xE6 => set_4_hl(cpu, bus),
        0xE7 => set_4_a(cpu),
        0xE8 => set_5_b(cpu),
        0xE9 => set_5_c(cpu),
        0xEA => set_5_d(cpu),
        0xEB => set_5_e(cpu),
        0xEC => set_5_h(cpu),
        0xED => set_5_l(cpu),
        0xEE => set_5_hl(cpu, bus),
        0xEF => set_5_a(cpu),
        0xF0 => set_6_b(cpu),
        0xF1 => set_6_c(cpu),
        0xF2 => set_6_d(cpu),
        0xF3 => set_6_e(cpu),
        0xF4 => set_6_h(cpu),
        0xF5 => set_6_l(cpu),
        0xF6 => set_6_hl(cpu, bus),
        0xF7 => set_6_a(cpu),
        0xF8 => set_7_b(cpu),
        0xF9 => set_7_c(cpu),
        0xFA => set_7_d(cpu),
        0xFB => set_7_e(cpu),
        0xFC => set_7_h(cpu),
        0xFD => set_7_l(cpu),
        0xFE => set_7_hl(cpu, bus),
        0xFF => set_7_a(cpu),
    }
}

fn rlc(cpu: &mut Cpu, value: Byte) -> Byte {
    let carry: bool = value & 0x80 != 0;
    let result: Byte = value.rotate_left(1);
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = carry;
    result
}

fn rrc(cpu: &mut Cpu, value: Byte) -> Byte {
    let carry: bool = value & 0x01 != 0;
    let result: Byte = value.rotate_right(1);
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = carry;
    result
}

fn rl(cpu: &mut Cpu, value: Byte) -> Byte {
    let old_carry = cpu.registers.f.c as Byte;
    let new_carry = value & 0x80 != 0;
    let result: Byte = (value << 1) | old_carry;
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = new_carry;
    result
}

fn rr(cpu: &mut Cpu, value: Byte) -> Byte {
    let old_carry = cpu.registers.f.c as Byte;
    let new_carry = value & 0x01 != 0;
    let result: Byte = (value >> 1) | (old_carry << 7);
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = new_carry;
    result
}

fn sla(cpu: &mut Cpu, value: Byte) -> Byte {
    let carry: bool = value & 0x80 != 0;
    let result: Byte = value << 1;
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = carry;
    result
}

fn sra(cpu: &mut Cpu, value: Byte) -> Byte {
    let carry: bool = value & 0x01 != 0;
    let result: Byte = (value >> 1) | (value & 0x80);
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = carry;
    result
}

fn swap(cpu: &mut Cpu, value: Byte) -> Byte {
    let result: Byte = value.rotate_right(4);
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = false;
    result
}

fn srl(cpu: &mut Cpu, value: Byte) -> Byte {
    let carry: bool = value & 0x01 != 0;
    let result: Byte = value >> 1;
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = carry;
    result
}

fn bit(cpu: &mut Cpu, bit_index: u8, value: Byte) {
    cpu.registers.f.z = value & (1 << bit_index) == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = true;
}

fn set_bit(bit_index: u8, value: Byte) -> Byte {
    value | (1 << bit_index)
}

fn res_bit(bit_index: u8, value: Byte) -> Byte {
    value & !(1 << bit_index)
}

macro_rules! cb_r8 {
    ($name:ident, $core:ident, $reg:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            let value: Byte = cpu.registers.$reg;
            cpu.registers.$reg = $core(cpu, value);
            8
        }
    };
}

cb_r8!(rlc_b, rlc, b);
cb_r8!(rlc_c, rlc, c);
cb_r8!(rlc_d, rlc, d);
cb_r8!(rlc_e, rlc, e);
cb_r8!(rlc_h, rlc, h);
cb_r8!(rlc_l, rlc, l);
cb_r8!(rlc_a, rlc, a);
cb_r8!(rrc_b, rrc, b);
cb_r8!(rrc_c, rrc, c);
cb_r8!(rrc_d, rrc, d);
cb_r8!(rrc_e, rrc, e);
cb_r8!(rrc_h, rrc, h);
cb_r8!(rrc_l, rrc, l);
cb_r8!(rrc_a, rrc, a);
cb_r8!(rl_b, rl, b);
cb_r8!(rl_c, rl, c);
cb_r8!(rl_d, rl, d);
cb_r8!(rl_e, rl, e);
cb_r8!(rl_h, rl, h);
cb_r8!(rl_l, rl, l);
cb_r8!(rl_a, rl, a);
cb_r8!(rr_b, rr, b);
cb_r8!(rr_c, rr, c);
cb_r8!(rr_d, rr, d);
cb_r8!(rr_e, rr, e);
cb_r8!(rr_h, rr, h);
cb_r8!(rr_l, rr, l);
cb_r8!(rr_a, rr, a);
cb_r8!(sla_b, sla, b);
cb_r8!(sla_c, sla, c);
cb_r8!(sla_d, sla, d);
cb_r8!(sla_e, sla, e);
cb_r8!(sla_h, sla, h);
cb_r8!(sla_l, sla, l);
cb_r8!(sla_a, sla, a);
cb_r8!(sra_b, sra, b);
cb_r8!(sra_c, sra, c);
cb_r8!(sra_d, sra, d);
cb_r8!(sra_e, sra, e);
cb_r8!(sra_h, sra, h);
cb_r8!(sra_l, sra, l);
cb_r8!(sra_a, sra, a);
cb_r8!(swap_b, swap, b);
cb_r8!(swap_c, swap, c);
cb_r8!(swap_d, swap, d);
cb_r8!(swap_e, swap, e);
cb_r8!(swap_h, swap, h);
cb_r8!(swap_l, swap, l);
cb_r8!(swap_a, swap, a);
cb_r8!(srl_b, srl, b);
cb_r8!(srl_c, srl, c);
cb_r8!(srl_d, srl, d);
cb_r8!(srl_e, srl, e);
cb_r8!(srl_h, srl, h);
cb_r8!(srl_l, srl, l);
cb_r8!(srl_a, srl, a);

macro_rules! cb_hl {
    ($name:ident, $core:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let address: Address = cpu.registers.hl();
            let value: Byte = bus.read(address);
            let result: Byte = $core(cpu, value);
            bus.write(address, result);
            16
        }
    };
}

cb_hl!(rlc_hl, rlc);
cb_hl!(rrc_hl, rrc);
cb_hl!(rl_hl, rl);
cb_hl!(rr_hl, rr);
cb_hl!(sla_hl, sla);
cb_hl!(sra_hl, sra);
cb_hl!(swap_hl, swap);
cb_hl!(srl_hl, srl);

macro_rules! bit_b_r8 {
    ($name:ident, $bit:expr, $reg:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            let value: Byte = cpu.registers.$reg;
            bit(cpu, $bit, value);
            8
        }
    };
}

bit_b_r8!(bit_0_b, 0, b);
bit_b_r8!(bit_0_c, 0, c);
bit_b_r8!(bit_0_d, 0, d);
bit_b_r8!(bit_0_e, 0, e);
bit_b_r8!(bit_0_h, 0, h);
bit_b_r8!(bit_0_l, 0, l);
bit_b_r8!(bit_0_a, 0, a);
bit_b_r8!(bit_1_b, 1, b);
bit_b_r8!(bit_1_c, 1, c);
bit_b_r8!(bit_1_d, 1, d);
bit_b_r8!(bit_1_e, 1, e);
bit_b_r8!(bit_1_h, 1, h);
bit_b_r8!(bit_1_l, 1, l);
bit_b_r8!(bit_1_a, 1, a);
bit_b_r8!(bit_2_b, 2, b);
bit_b_r8!(bit_2_c, 2, c);
bit_b_r8!(bit_2_d, 2, d);
bit_b_r8!(bit_2_e, 2, e);
bit_b_r8!(bit_2_h, 2, h);
bit_b_r8!(bit_2_l, 2, l);
bit_b_r8!(bit_2_a, 2, a);
bit_b_r8!(bit_3_b, 3, b);
bit_b_r8!(bit_3_c, 3, c);
bit_b_r8!(bit_3_d, 3, d);
bit_b_r8!(bit_3_e, 3, e);
bit_b_r8!(bit_3_h, 3, h);
bit_b_r8!(bit_3_l, 3, l);
bit_b_r8!(bit_3_a, 3, a);
bit_b_r8!(bit_4_b, 4, b);
bit_b_r8!(bit_4_c, 4, c);
bit_b_r8!(bit_4_d, 4, d);
bit_b_r8!(bit_4_e, 4, e);
bit_b_r8!(bit_4_h, 4, h);
bit_b_r8!(bit_4_l, 4, l);
bit_b_r8!(bit_4_a, 4, a);
bit_b_r8!(bit_5_b, 5, b);
bit_b_r8!(bit_5_c, 5, c);
bit_b_r8!(bit_5_d, 5, d);
bit_b_r8!(bit_5_e, 5, e);
bit_b_r8!(bit_5_h, 5, h);
bit_b_r8!(bit_5_l, 5, l);
bit_b_r8!(bit_5_a, 5, a);
bit_b_r8!(bit_6_b, 6, b);
bit_b_r8!(bit_6_c, 6, c);
bit_b_r8!(bit_6_d, 6, d);
bit_b_r8!(bit_6_e, 6, e);
bit_b_r8!(bit_6_h, 6, h);
bit_b_r8!(bit_6_l, 6, l);
bit_b_r8!(bit_6_a, 6, a);
bit_b_r8!(bit_7_b, 7, b);
bit_b_r8!(bit_7_c, 7, c);
bit_b_r8!(bit_7_d, 7, d);
bit_b_r8!(bit_7_e, 7, e);
bit_b_r8!(bit_7_h, 7, h);
bit_b_r8!(bit_7_l, 7, l);
bit_b_r8!(bit_7_a, 7, a);

macro_rules! bit_b_hl {
    ($name:ident, $bit:expr) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let value: Byte = bus.read(cpu.registers.hl());
            bit(cpu, $bit, value);
            12
        }
    };
}

bit_b_hl!(bit_0_hl, 0);
bit_b_hl!(bit_1_hl, 1);
bit_b_hl!(bit_2_hl, 2);
bit_b_hl!(bit_3_hl, 3);
bit_b_hl!(bit_4_hl, 4);
bit_b_hl!(bit_5_hl, 5);
bit_b_hl!(bit_6_hl, 6);
bit_b_hl!(bit_7_hl, 7);

macro_rules! res_b_r8 {
    ($name:ident, $bit:expr, $reg:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            cpu.registers.$reg = res_bit($bit, cpu.registers.$reg);
            8
        }
    };
}

res_b_r8!(res_0_b, 0, b);
res_b_r8!(res_0_c, 0, c);
res_b_r8!(res_0_d, 0, d);
res_b_r8!(res_0_e, 0, e);
res_b_r8!(res_0_h, 0, h);
res_b_r8!(res_0_l, 0, l);
res_b_r8!(res_0_a, 0, a);
res_b_r8!(res_1_b, 1, b);
res_b_r8!(res_1_c, 1, c);
res_b_r8!(res_1_d, 1, d);
res_b_r8!(res_1_e, 1, e);
res_b_r8!(res_1_h, 1, h);
res_b_r8!(res_1_l, 1, l);
res_b_r8!(res_1_a, 1, a);
res_b_r8!(res_2_b, 2, b);
res_b_r8!(res_2_c, 2, c);
res_b_r8!(res_2_d, 2, d);
res_b_r8!(res_2_e, 2, e);
res_b_r8!(res_2_h, 2, h);
res_b_r8!(res_2_l, 2, l);
res_b_r8!(res_2_a, 2, a);
res_b_r8!(res_3_b, 3, b);
res_b_r8!(res_3_c, 3, c);
res_b_r8!(res_3_d, 3, d);
res_b_r8!(res_3_e, 3, e);
res_b_r8!(res_3_h, 3, h);
res_b_r8!(res_3_l, 3, l);
res_b_r8!(res_3_a, 3, a);
res_b_r8!(res_4_b, 4, b);
res_b_r8!(res_4_c, 4, c);
res_b_r8!(res_4_d, 4, d);
res_b_r8!(res_4_e, 4, e);
res_b_r8!(res_4_h, 4, h);
res_b_r8!(res_4_l, 4, l);
res_b_r8!(res_4_a, 4, a);
res_b_r8!(res_5_b, 5, b);
res_b_r8!(res_5_c, 5, c);
res_b_r8!(res_5_d, 5, d);
res_b_r8!(res_5_e, 5, e);
res_b_r8!(res_5_h, 5, h);
res_b_r8!(res_5_l, 5, l);
res_b_r8!(res_5_a, 5, a);
res_b_r8!(res_6_b, 6, b);
res_b_r8!(res_6_c, 6, c);
res_b_r8!(res_6_d, 6, d);
res_b_r8!(res_6_e, 6, e);
res_b_r8!(res_6_h, 6, h);
res_b_r8!(res_6_l, 6, l);
res_b_r8!(res_6_a, 6, a);
res_b_r8!(res_7_b, 7, b);
res_b_r8!(res_7_c, 7, c);
res_b_r8!(res_7_d, 7, d);
res_b_r8!(res_7_e, 7, e);
res_b_r8!(res_7_h, 7, h);
res_b_r8!(res_7_l, 7, l);
res_b_r8!(res_7_a, 7, a);

macro_rules! res_b_hl {
    ($name:ident, $bit:expr) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let address: Address = cpu.registers.hl();
            let value: Byte = bus.read(address);
            bus.write(address, res_bit($bit, value));
            16
        }
    };
}

res_b_hl!(res_0_hl, 0);
res_b_hl!(res_1_hl, 1);
res_b_hl!(res_2_hl, 2);
res_b_hl!(res_3_hl, 3);
res_b_hl!(res_4_hl, 4);
res_b_hl!(res_5_hl, 5);
res_b_hl!(res_6_hl, 6);
res_b_hl!(res_7_hl, 7);

macro_rules! set_b_r8 {
    ($name:ident, $bit:expr, $reg:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            cpu.registers.$reg = set_bit($bit, cpu.registers.$reg);
            8
        }
    };
}

set_b_r8!(set_0_b, 0, b);
set_b_r8!(set_0_c, 0, c);
set_b_r8!(set_0_d, 0, d);
set_b_r8!(set_0_e, 0, e);
set_b_r8!(set_0_h, 0, h);
set_b_r8!(set_0_l, 0, l);
set_b_r8!(set_0_a, 0, a);
set_b_r8!(set_1_b, 1, b);
set_b_r8!(set_1_c, 1, c);
set_b_r8!(set_1_d, 1, d);
set_b_r8!(set_1_e, 1, e);
set_b_r8!(set_1_h, 1, h);
set_b_r8!(set_1_l, 1, l);
set_b_r8!(set_1_a, 1, a);
set_b_r8!(set_2_b, 2, b);
set_b_r8!(set_2_c, 2, c);
set_b_r8!(set_2_d, 2, d);
set_b_r8!(set_2_e, 2, e);
set_b_r8!(set_2_h, 2, h);
set_b_r8!(set_2_l, 2, l);
set_b_r8!(set_2_a, 2, a);
set_b_r8!(set_3_b, 3, b);
set_b_r8!(set_3_c, 3, c);
set_b_r8!(set_3_d, 3, d);
set_b_r8!(set_3_e, 3, e);
set_b_r8!(set_3_h, 3, h);
set_b_r8!(set_3_l, 3, l);
set_b_r8!(set_3_a, 3, a);
set_b_r8!(set_4_b, 4, b);
set_b_r8!(set_4_c, 4, c);
set_b_r8!(set_4_d, 4, d);
set_b_r8!(set_4_e, 4, e);
set_b_r8!(set_4_h, 4, h);
set_b_r8!(set_4_l, 4, l);
set_b_r8!(set_4_a, 4, a);
set_b_r8!(set_5_b, 5, b);
set_b_r8!(set_5_c, 5, c);
set_b_r8!(set_5_d, 5, d);
set_b_r8!(set_5_e, 5, e);
set_b_r8!(set_5_h, 5, h);
set_b_r8!(set_5_l, 5, l);
set_b_r8!(set_5_a, 5, a);
set_b_r8!(set_6_b, 6, b);
set_b_r8!(set_6_c, 6, c);
set_b_r8!(set_6_d, 6, d);
set_b_r8!(set_6_e, 6, e);
set_b_r8!(set_6_h, 6, h);
set_b_r8!(set_6_l, 6, l);
set_b_r8!(set_6_a, 6, a);
set_b_r8!(set_7_b, 7, b);
set_b_r8!(set_7_c, 7, c);
set_b_r8!(set_7_d, 7, d);
set_b_r8!(set_7_e, 7, e);
set_b_r8!(set_7_h, 7, h);
set_b_r8!(set_7_l, 7, l);
set_b_r8!(set_7_a, 7, a);

macro_rules! set_b_hl {
    ($name:ident, $bit:expr) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let address: Address = cpu.registers.hl();
            let value: Byte = bus.read(address);
            bus.write(address, set_bit($bit, value));
            16
        }
    };
}

set_b_hl!(set_0_hl, 0);
set_b_hl!(set_1_hl, 1);
set_b_hl!(set_2_hl, 2);
set_b_hl!(set_3_hl, 3);
set_b_hl!(set_4_hl, 4);
set_b_hl!(set_5_hl, 5);
set_b_hl!(set_6_hl, 6);
set_b_hl!(set_7_hl, 7);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_helpers::FlatRam;
    use crate::cpu::Cpu;

    #[test]
    fn rlc_b_rotates_and_sets_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b1000_0001;
        rlc_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0b0000_0011);
        assert!(cpu.registers.f.c);
        assert!(!cpu.registers.f.z);
    }

    #[test]
    fn rlc_b_sets_zero_flag_on_zero_result() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x00;
        rlc_b(&mut cpu);
        assert!(cpu.registers.f.z);
    }

    #[test]
    fn sla_b_shifts_and_clears_bit0() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b1000_0001;
        sla_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0b0000_0010);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn sra_b_preserves_sign_bit() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b1000_0001;
        sra_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0b1100_0000);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn srl_b_clears_bit7() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b1000_0001;
        srl_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0b0100_0000);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn swap_b_exchanges_nibbles() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0xAB;
        swap_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0xBA);
        assert!(!cpu.registers.f.c);
    }

    #[test]
    fn swap_b_sets_zero_flag_on_zero() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x00;
        swap_b(&mut cpu);
        assert!(cpu.registers.f.z);
    }

    #[test]
    fn bit_7_b_sets_zero_when_bit_clear() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b0111_1111;
        bit_7_b(&mut cpu);
        assert!(cpu.registers.f.z);
        assert!(!cpu.registers.f.n);
        assert!(cpu.registers.f.h);
    }

    #[test]
    fn bit_7_b_clears_zero_when_bit_set() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b1000_0000;
        bit_7_b(&mut cpu);
        assert!(!cpu.registers.f.z);
    }

    #[test]
    fn bit_does_not_modify_value_or_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b1000_0000;
        cpu.registers.f.c = true;
        bit_7_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0b1000_0000);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn res_0_b_clears_bit_without_touching_flags() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b1111_1111;
        let flags_before = cpu.registers.f.to_byte();
        res_0_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0b1111_1110);
        assert_eq!(cpu.registers.f.to_byte(), flags_before);
    }

    #[test]
    fn set_0_b_sets_bit_without_touching_flags() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0b0000_0000;
        let flags_before = cpu.registers.f.to_byte();
        set_0_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0b0000_0001);
        assert_eq!(cpu.registers.f.to_byte(), flags_before);
    }

    #[test]
    fn rlc_hl_operates_on_memory_not_register() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.set_hl(0xC000);
        bus.write(0xC000, 0b1000_0001);
        let cycles = rlc_hl(&mut cpu, &mut bus);
        assert_eq!(bus.read(0xC000), 0b0000_0011);
        assert_eq!(cpu.registers.hl(), 0xC000);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn bit_7_hl_costs_12_cycles_not_16() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.set_hl(0xC000);
        bus.write(0xC000, 0x00);
        let cycles = bit_7_hl(&mut cpu, &mut bus);
        assert_eq!(cycles, 12);
    }

    #[test]
    fn set_7_hl_costs_16_cycles() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.set_hl(0xC000);
        bus.write(0xC000, 0x00);
        let cycles = set_7_hl(&mut cpu, &mut bus);
        assert_eq!(bus.read(0xC000), 0b1000_0000);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn execute_dispatches_rlc_b() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.b = 0b1000_0001;
        let cycles = execute(&mut cpu, &mut bus, 0x00);
        assert_eq!(cpu.registers.b, 0b0000_0011);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn execute_dispatches_bit_7_a() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.a = 0x00;
        let cycles = execute(&mut cpu, &mut bus, 0x7F);
        assert!(cpu.registers.f.z);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn execute_dispatches_set_7_a() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.a = 0x00;
        let cycles = execute(&mut cpu, &mut bus, 0xFF);
        assert_eq!(cpu.registers.a, 0b1000_0000);
        assert_eq!(cycles, 8);
    }
}
