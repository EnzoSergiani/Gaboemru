use crate::common::types::{Address, Byte, Cycles, Word};

use super::{Bus, Cpu};

fn add_with_carry(cpu: &mut Cpu, value: Byte, carry_in: bool) -> Byte {
    let a: Byte = cpu.registers.a;
    let carry_in: Byte = carry_in as Byte;
    let (partial, c1): (Byte, bool) = a.overflowing_add(value);
    let (result, c2): (Byte, bool) = partial.overflowing_add(carry_in);
    let half_carry: bool = (a & 0x0F) + (value & 0x0F) + carry_in > 0x0F;
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = half_carry;
    cpu.registers.f.c = c1 || c2;
    cpu.registers.a = result;
    result
}

fn add(cpu: &mut Cpu, value: Byte) -> Byte {
    add_with_carry(cpu, value, false)
}

fn adc(cpu: &mut Cpu, value: Byte) -> Byte {
    add_with_carry(cpu, value, cpu.registers.f.c)
}

fn sub_with_carry(cpu: &mut Cpu, value: Byte, carry_in: bool) -> Byte {
    let a: Byte = cpu.registers.a;
    let carry_in: Byte = carry_in as Byte;
    let (partial, b1): (Byte, bool) = a.overflowing_sub(value);
    let (result, b2): (Byte, bool) = partial.overflowing_sub(carry_in);
    let half_carry: bool = (a & 0x0F) < (value & 0x0F) + carry_in;
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = true;
    cpu.registers.f.h = half_carry;
    cpu.registers.f.c = b1 || b2;
    cpu.registers.a = result;
    result
}

fn sub(cpu: &mut Cpu, value: Byte) -> Byte {
    sub_with_carry(cpu, value, false)
}

fn sbc(cpu: &mut Cpu, value: Byte) -> Byte {
    sub_with_carry(cpu, value, cpu.registers.f.c)
}

fn cp(cpu: &mut Cpu, value: Byte) {
    let a: Byte = cpu.registers.a;
    let (result, carry): (Byte, bool) = a.overflowing_sub(value);
    let half_carry: bool = (a & 0x0F) < (value & 0x0F);

    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = true;
    cpu.registers.f.h = half_carry;
    cpu.registers.f.c = carry;
}

fn and(cpu: &mut Cpu, value: Byte) {
    cpu.registers.a &= value;
    cpu.registers.f.z = cpu.registers.a == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = true;
    cpu.registers.f.c = false;
}

fn xor(cpu: &mut Cpu, value: Byte) {
    cpu.registers.a ^= value;
    cpu.registers.f.z = cpu.registers.a == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = false;
}

fn or(cpu: &mut Cpu, value: Byte) {
    cpu.registers.a |= value;
    cpu.registers.f.z = cpu.registers.a == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = false;
}

macro_rules! alu_r8 {
    ($name:ident, $core:ident, $src:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            let value: Byte = cpu.registers.$src;
            $core(cpu, value);
            4
        }
    };
}

alu_r8!(add_a_a, add, a);
alu_r8!(add_a_b, add, b);
alu_r8!(add_a_c, add, c);
alu_r8!(add_a_d, add, d);
alu_r8!(add_a_e, add, e);
alu_r8!(add_a_h, add, h);
alu_r8!(add_a_l, add, l);

alu_r8!(adc_a_a, adc, a);
alu_r8!(adc_a_b, adc, b);
alu_r8!(adc_a_c, adc, c);
alu_r8!(adc_a_d, adc, d);
alu_r8!(adc_a_e, adc, e);
alu_r8!(adc_a_h, adc, h);
alu_r8!(adc_a_l, adc, l);

alu_r8!(sub_a_a, sub, a);
alu_r8!(sub_a_b, sub, b);
alu_r8!(sub_a_c, sub, c);
alu_r8!(sub_a_d, sub, d);
alu_r8!(sub_a_e, sub, e);
alu_r8!(sub_a_h, sub, h);
alu_r8!(sub_a_l, sub, l);

alu_r8!(sbc_a_a, sbc, a);
alu_r8!(sbc_a_b, sbc, b);
alu_r8!(sbc_a_c, sbc, c);
alu_r8!(sbc_a_d, sbc, d);
alu_r8!(sbc_a_e, sbc, e);
alu_r8!(sbc_a_h, sbc, h);
alu_r8!(sbc_a_l, sbc, l);

macro_rules! alu_r8_void {
    ($name:ident, $core:ident, $src:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            let value: Byte = cpu.registers.$src;
            $core(cpu, value);
            4
        }
    };
}

alu_r8_void!(and_a_a, and, a);
alu_r8_void!(and_a_b, and, b);
alu_r8_void!(and_a_c, and, c);
alu_r8_void!(and_a_d, and, d);
alu_r8_void!(and_a_e, and, e);
alu_r8_void!(and_a_h, and, h);
alu_r8_void!(and_a_l, and, l);

alu_r8_void!(xor_a_a, xor, a);
alu_r8_void!(xor_a_b, xor, b);
alu_r8_void!(xor_a_c, xor, c);
alu_r8_void!(xor_a_d, xor, d);
alu_r8_void!(xor_a_e, xor, e);
alu_r8_void!(xor_a_h, xor, h);
alu_r8_void!(xor_a_l, xor, l);

alu_r8_void!(or_a_a, or, a);
alu_r8_void!(or_a_b, or, b);
alu_r8_void!(or_a_c, or, c);
alu_r8_void!(or_a_d, or, d);
alu_r8_void!(or_a_e, or, e);
alu_r8_void!(or_a_h, or, h);
alu_r8_void!(or_a_l, or, l);

alu_r8_void!(cp_a_a, cp, a);
alu_r8_void!(cp_a_b, cp, b);
alu_r8_void!(cp_a_c, cp, c);
alu_r8_void!(cp_a_d, cp, d);
alu_r8_void!(cp_a_e, cp, e);
alu_r8_void!(cp_a_h, cp, h);
alu_r8_void!(cp_a_l, cp, l);

macro_rules! alu_hl {
    ($name:ident, $core:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let value: Byte = bus.read(cpu.registers.hl());
            $core(cpu, value);
            8
        }
    };
}

alu_hl!(add_a_hl, add);
alu_hl!(adc_a_hl, adc);
alu_hl!(sub_a_hl, sub);
alu_hl!(sbc_a_hl, sbc);
alu_hl!(and_a_hl, and);
alu_hl!(xor_a_hl, xor);
alu_hl!(or_a_hl, or);
alu_hl!(cp_a_hl, cp);

macro_rules! alu_n8 {
    ($name:ident, $core:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let value: Byte = cpu.fetch_byte(bus);
            $core(cpu, value);
            8
        }
    };
}

alu_n8!(add_a_n8, add);
alu_n8!(adc_a_n8, adc);
alu_n8!(sub_a_n8, sub);
alu_n8!(sbc_a_n8, sbc);
alu_n8!(and_a_n8, and);
alu_n8!(xor_a_n8, xor);
alu_n8!(or_a_n8, or);
alu_n8!(cp_a_n8, cp);

macro_rules! inc_r8 {
    ($name:ident, $reg:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            let old: Byte = cpu.registers.$reg;
            let result: Byte = old.wrapping_add(1);
            cpu.registers.f.z = result == 0;
            cpu.registers.f.n = false;
            cpu.registers.f.h = (old & 0x0F) == 0x0F;
            cpu.registers.$reg = result;
            4
        }
    };
}

inc_r8!(inc_a, a);
inc_r8!(inc_b, b);
inc_r8!(inc_c, c);
inc_r8!(inc_d, d);
inc_r8!(inc_e, e);
inc_r8!(inc_h, h);
inc_r8!(inc_l, l);

macro_rules! dec_r8 {
    ($name:ident, $reg:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            let old: Byte = cpu.registers.$reg;
            let result: Byte = old.wrapping_sub(1);
            cpu.registers.f.z = result == 0;
            cpu.registers.f.n = true;
            cpu.registers.f.h = (old & 0x0F) == 0x00;
            cpu.registers.$reg = result;
            4
        }
    };
}

dec_r8!(dec_a, a);
dec_r8!(dec_b, b);
dec_r8!(dec_c, c);
dec_r8!(dec_d, d);
dec_r8!(dec_e, e);
dec_r8!(dec_h, h);
dec_r8!(dec_l, l);

pub fn inc_hl<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.registers.hl();
    let old: Byte = bus.read(address);
    let result: Byte = old.wrapping_add(1);
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = false;
    cpu.registers.f.h = (old & 0x0F) == 0x0F;
    bus.write(address, result);
    12
}

pub fn dec_hl<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.registers.hl();
    let old: Byte = bus.read(address);
    let result: Byte = old.wrapping_sub(1);
    cpu.registers.f.z = result == 0;
    cpu.registers.f.n = true;
    cpu.registers.f.h = (old & 0x0F) == 0x00;
    bus.write(address, result);
    12
}

macro_rules! inc_r16 {
    ($name:ident, $getter:ident, $setter:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            let value: Word = cpu.registers.$getter().wrapping_add(1);
            cpu.registers.$setter(value);
            8
        }
    };
}

inc_r16!(inc_bc, bc, set_bc);
inc_r16!(inc_de, de, set_de);
inc_r16!(inc_hl16, hl, set_hl);

pub fn inc_sp(cpu: &mut Cpu) -> Cycles {
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    8
}

macro_rules! dec_r16 {
    ($name:ident, $getter:ident, $setter:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            let value: Word = cpu.registers.$getter().wrapping_sub(1);
            cpu.registers.$setter(value);
            8
        }
    };
}

dec_r16!(dec_bc, bc, set_bc);
dec_r16!(dec_de, de, set_de);
dec_r16!(dec_hl16, hl, set_hl);

pub fn dec_sp(cpu: &mut Cpu) -> Cycles {
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    8
}

macro_rules! add_hl_r16 {
    ($name:ident, $getter:ident) => {
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            let hl: Word = cpu.registers.hl();
            let value: Word = cpu.registers.$getter();
            let (result, carry): (Word, bool) = hl.overflowing_add(value);
            let half_carry: bool = (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
            cpu.registers.f.n = false;
            cpu.registers.f.h = half_carry;
            cpu.registers.f.c = carry;
            cpu.registers.set_hl(result);
            8
        }
    };
}

add_hl_r16!(add_hl_bc, bc);
add_hl_r16!(add_hl_de, de);
add_hl_r16!(add_hl_hl, hl);

pub fn add_hl_sp(cpu: &mut Cpu) -> Cycles {
    let hl: Word = cpu.registers.hl();
    let value: Word = cpu.registers.sp;
    let (result, carry): (Word, bool) = hl.overflowing_add(value);
    let half_carry: bool = (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
    cpu.registers.f.n = false;
    cpu.registers.f.h = half_carry;
    cpu.registers.f.c = carry;
    cpu.registers.set_hl(result);
    8
}

pub fn add_sp_e8<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let offset = cpu.fetch_byte(bus) as i8 as i16;
    let sp: Word = cpu.registers.sp;
    let result: Word = (sp as i16).wrapping_add(offset) as Word;
    let half_carry: bool = (sp & 0x0F) + (offset as Word & 0x0F) > 0x0F;
    let carry: bool = (sp & 0xFF) + (offset as Word & 0xFF) > 0xFF;
    cpu.registers.f.z = false;
    cpu.registers.f.n = false;
    cpu.registers.f.h = half_carry;
    cpu.registers.f.c = carry;
    cpu.registers.sp = result;
    16
}

pub fn daa(cpu: &mut Cpu) -> Cycles {
    let mut a: Byte = cpu.registers.a;
    let mut adjust: Byte = 0;
    let mut carry: bool = cpu.registers.f.c;
    if cpu.registers.f.n {
        if cpu.registers.f.h {
            adjust |= 0x06;
        }
        if cpu.registers.f.c {
            adjust |= 0x60;
        }
        a = a.wrapping_sub(adjust);
    } else {
        if cpu.registers.f.h || (a & 0x0F) > 0x09 {
            adjust |= 0x06;
        }
        if cpu.registers.f.c || a > 0x99 {
            adjust |= 0x60;
            carry = true;
        }
        a = a.wrapping_add(adjust);
    }
    cpu.registers.f.z = a == 0;
    cpu.registers.f.h = false;
    cpu.registers.f.c = carry;
    cpu.registers.a = a;
    4
}

pub fn cpl(cpu: &mut Cpu) -> Cycles {
    cpu.registers.a = !cpu.registers.a;
    cpu.registers.f.n = true;
    cpu.registers.f.h = true;
    4
}

pub fn scf(cpu: &mut Cpu) -> Cycles {
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = true;
    4
}

pub fn ccf(cpu: &mut Cpu) -> Cycles {
    cpu.registers.f.n = false;
    cpu.registers.f.h = false;
    cpu.registers.f.c = !cpu.registers.f.c;
    4
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_helpers::FlatRam;
    use crate::cpu::Cpu;

    #[test]
    fn add_a_b_basic() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x05;
        cpu.registers.b = 0x03;
        let cycles = add_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x08);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn add_a_b_sets_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x00;
        cpu.registers.b = 0x00;
        add_a_b(&mut cpu);
        assert!(cpu.registers.f.z);
        assert!(!cpu.registers.f.n);
        assert!(!cpu.registers.f.h);
        assert!(!cpu.registers.f.c);
    }

    #[test]
    fn add_a_b_sets_carry_on_overflow() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.registers.b = 0x01;
        add_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.f.z);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn add_a_b_sets_half_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x0F;
        cpu.registers.b = 0x01;
        add_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x10);
        assert!(cpu.registers.f.h);
        assert!(!cpu.registers.f.c);
    }

    #[test]
    fn add_a_b_clears_subtract_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.f.n = true;
        add_a_b(&mut cpu);
        assert!(!cpu.registers.f.n);
    }

    #[test]
    fn add_a_hl_reads_memory_and_costs_8_cycles() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.a = 0x05;
        cpu.registers.set_hl(0xC000);
        bus.write(0xC000, 0x03);

        let cycles = add_a_hl(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.a, 0x08);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn add_a_n8_reads_immediate_and_costs_8_cycles() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.a = 0x05;
        bus.write(cpu.registers.pc, 0x03);

        let cycles = add_a_n8(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.a, 0x08);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn adc_a_b_adds_carry_in() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x05;
        cpu.registers.b = 0x03;
        cpu.registers.f.c = true;
        adc_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x09);
    }

    #[test]
    fn adc_a_b_without_carry_in_behaves_like_add() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x05;
        cpu.registers.b = 0x03;
        cpu.registers.f.c = false;
        adc_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x08);
    }

    #[test]
    fn adc_a_b_carry_propagates_through_both_additions() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.registers.b = 0x00;
        cpu.registers.f.c = true;
        adc_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.f.z);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn sub_a_b_basic() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x08;
        cpu.registers.b = 0x03;
        let cycles = sub_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x05);
        assert!(cpu.registers.f.n);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn sub_a_b_sets_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x05;
        cpu.registers.b = 0x05;
        sub_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.f.z);
    }

    #[test]
    fn sub_a_b_sets_carry_on_borrow() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x00;
        cpu.registers.b = 0x01;
        sub_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0xFF);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn sub_a_b_sets_half_carry_on_nibble_borrow() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x10;
        cpu.registers.b = 0x01;
        sub_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x0F);
        assert!(cpu.registers.f.h);
    }

    #[test]
    fn sbc_a_b_subtracts_carry_in() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x08;
        cpu.registers.b = 0x03;
        cpu.registers.f.c = true;
        sbc_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x04);
    }

    #[test]
    fn cp_a_b_does_not_modify_a() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x08;
        cpu.registers.b = 0x03;
        let cycles = cp_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0x08);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn cp_a_b_sets_zero_flag_when_equal() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x05;
        cpu.registers.b = 0x05;
        cp_a_b(&mut cpu);
        assert!(cpu.registers.f.z);
        assert!(cpu.registers.f.n);
    }

    #[test]
    fn cp_a_b_sets_carry_when_a_less_than_b() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x03;
        cpu.registers.b = 0x05;
        cp_a_b(&mut cpu);
        assert!(cpu.registers.f.c);
        assert!(!cpu.registers.f.z);
    }

    #[test]
    fn and_a_b_sets_half_carry_always() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b1100;
        cpu.registers.b = 0b1010;
        and_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0b1000);
        assert!(cpu.registers.f.h);
        assert!(!cpu.registers.f.c);
        assert!(!cpu.registers.f.n);
    }

    #[test]
    fn and_a_b_sets_zero_flag_when_result_zero() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b1100;
        cpu.registers.b = 0b0011;
        and_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0);
        assert!(cpu.registers.f.z);
    }

    #[test]
    fn or_a_b_clears_half_carry_and_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b1100;
        cpu.registers.b = 0b0011;
        or_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0b1111);
        assert!(!cpu.registers.f.h);
        assert!(!cpu.registers.f.c);
        assert!(!cpu.registers.f.n);
    }

    #[test]
    fn xor_a_a_zeroes_register_and_sets_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x42;
        xor_a_a(&mut cpu);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.f.z);
        assert!(!cpu.registers.f.n);
        assert!(!cpu.registers.f.h);
        assert!(!cpu.registers.f.c);
    }

    #[test]
    fn xor_a_b_toggles_bits() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b1100;
        cpu.registers.b = 0b1010;
        xor_a_b(&mut cpu);
        assert_eq!(cpu.registers.a, 0b0110);
    }

    #[test]
    fn inc_b_basic() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x05;
        let cycles = inc_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0x06);
        assert!(!cpu.registers.f.z);
        assert!(!cpu.registers.f.n);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn inc_b_wraps_and_sets_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0xFF;
        inc_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0x00);
        assert!(cpu.registers.f.z);
        assert!(cpu.registers.f.h);
    }

    #[test]
    fn inc_b_does_not_affect_carry_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0xFF;
        cpu.registers.f.c = false;
        inc_b(&mut cpu);
        assert!(!cpu.registers.f.c);

        cpu.registers.f.c = true;
        cpu.registers.b = 0xFF;
        inc_b(&mut cpu);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn dec_b_basic() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x05;
        let cycles = dec_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0x04);
        assert!(cpu.registers.f.n);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn dec_b_wraps_and_sets_half_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x00;
        dec_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0xFF);
        assert!(cpu.registers.f.h);
        assert!(!cpu.registers.f.z);
    }

    #[test]
    fn dec_b_sets_zero_flag_when_reaching_zero() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x01;
        dec_b(&mut cpu);
        assert_eq!(cpu.registers.b, 0x00);
        assert!(cpu.registers.f.z);
    }

    #[test]
    fn inc_hl_increments_memory_not_register() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.set_hl(0xC000);
        bus.write(0xC000, 0x05);

        let cycles = inc_hl(&mut cpu, &mut bus);

        assert_eq!(bus.read(0xC000), 0x06);
        assert_eq!(cpu.registers.hl(), 0xC000);
        assert_eq!(cycles, 12);
    }

    #[test]
    fn dec_hl_decrements_memory_not_register() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.set_hl(0xC000);
        bus.write(0xC000, 0x05);

        let cycles = dec_hl(&mut cpu, &mut bus);

        assert_eq!(bus.read(0xC000), 0x04);
        assert_eq!(cpu.registers.hl(), 0xC000);
        assert_eq!(cycles, 12);
    }

    #[test]
    fn inc_bc_increments_register_not_memory() {
        let mut cpu = Cpu::new();
        cpu.registers.set_bc(0x1234);
        let flags_before = cpu.registers.f.to_byte();

        let cycles = inc_bc(&mut cpu);

        assert_eq!(cpu.registers.bc(), 0x1235);
        assert_eq!(cpu.registers.f.to_byte(), flags_before);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn inc_bc_wraps_at_0xffff() {
        let mut cpu = Cpu::new();
        cpu.registers.set_bc(0xFFFF);
        inc_bc(&mut cpu);
        assert_eq!(cpu.registers.bc(), 0x0000);
    }

    #[test]
    fn dec_bc_wraps_at_0x0000() {
        let mut cpu = Cpu::new();
        cpu.registers.set_bc(0x0000);
        dec_bc(&mut cpu);
        assert_eq!(cpu.registers.bc(), 0xFFFF);
    }

    #[test]
    fn inc_sp_and_dec_sp() {
        let mut cpu = Cpu::new();
        cpu.registers.sp = 0xFFFE;
        inc_sp(&mut cpu);
        assert_eq!(cpu.registers.sp, 0xFFFF);
        dec_sp(&mut cpu);
        assert_eq!(cpu.registers.sp, 0xFFFE);
    }

    #[test]
    fn add_hl_bc_basic() {
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1000);
        cpu.registers.set_bc(0x0100);
        let cycles = add_hl_bc(&mut cpu);
        assert_eq!(cpu.registers.hl(), 0x1100);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn add_hl_bc_does_not_affect_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.f.z = true;
        cpu.registers.set_hl(0x0000);
        cpu.registers.set_bc(0x0000);
        add_hl_bc(&mut cpu);
        assert!(cpu.registers.f.z);
    }

    #[test]
    fn add_hl_bc_sets_carry_on_16bit_overflow() {
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0xFFFF);
        cpu.registers.set_bc(0x0001);
        add_hl_bc(&mut cpu);
        assert_eq!(cpu.registers.hl(), 0x0000);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn add_hl_bc_sets_half_carry_on_bit11_overflow() {
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x0FFF);
        cpu.registers.set_bc(0x0001);
        add_hl_bc(&mut cpu);
        assert_eq!(cpu.registers.hl(), 0x1000);
        assert!(cpu.registers.f.h);
        assert!(!cpu.registers.f.c);
    }

    #[test]
    fn add_hl_hl_doubles_value() {
        let mut cpu = Cpu::new();
        cpu.registers.set_hl(0x1234);
        add_hl_hl(&mut cpu);
        assert_eq!(cpu.registers.hl(), 0x2468);
    }

    #[test]
    fn add_sp_e8_positive_offset() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.sp = 0xFFF8;
        bus.write(cpu.registers.pc, 0x02);

        let cycles = add_sp_e8(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.sp, 0xFFFA);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn add_sp_e8_negative_offset() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.sp = 0xFFF8;
        bus.write(cpu.registers.pc, 0xFE);

        add_sp_e8(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.sp, 0xFFF6);
    }

    #[test]
    fn add_sp_e8_always_clears_zero_flag() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.f.z = true;
        cpu.registers.sp = 0x0000;
        bus.write(cpu.registers.pc, 0x00);

        add_sp_e8(&mut cpu, &mut bus);
        assert!(!cpu.registers.f.z);
    }

    #[test]
    fn daa_after_addition_requiring_low_nibble_adjust() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x0A;
        cpu.registers.f.n = false;
        cpu.registers.f.h = false;
        cpu.registers.f.c = false;
        daa(&mut cpu);
        assert_eq!(cpu.registers.a, 0x10);
    }

    #[test]
    fn daa_after_addition_requiring_high_nibble_adjust() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xA0;
        cpu.registers.f.n = false;
        cpu.registers.f.h = false;
        cpu.registers.f.c = false;
        daa(&mut cpu);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn daa_known_bcd_addition() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x45;
        add_a_n8_direct(&mut cpu, 0x38);
        daa(&mut cpu);
        assert_eq!(cpu.registers.a, 0x83);
    }

    fn add_a_n8_direct(cpu: &mut Cpu, value: Byte) {
        add(cpu, value);
    }

    #[test]
    fn daa_after_subtraction() {
        // 0x83 - 0x38 en BCD = 0x45
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x83;
        sub(&mut cpu, 0x38);
        daa(&mut cpu);
        assert_eq!(cpu.registers.a, 0x45);
    }

    #[test]
    fn daa_always_clears_half_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x00;
        cpu.registers.f.h = true;
        daa(&mut cpu);
        assert!(!cpu.registers.f.h);
    }

    #[test]
    fn cpl_inverts_all_bits() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0b1010_0101;
        let cycles = cpl(&mut cpu);
        assert_eq!(cpu.registers.a, 0b0101_1010);
        assert!(cpu.registers.f.n);
        assert!(cpu.registers.f.h);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn cpl_does_not_affect_zero_or_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x00;
        cpu.registers.f.z = true;
        cpu.registers.f.c = true;
        cpl(&mut cpu);
        assert!(cpu.registers.f.z);
        assert!(cpu.registers.f.c);
    }

    #[test]
    fn scf_sets_carry_and_clears_n_h() {
        let mut cpu = Cpu::new();
        cpu.registers.f.n = true;
        cpu.registers.f.h = true;
        cpu.registers.f.c = false;
        let cycles = scf(&mut cpu);
        assert!(cpu.registers.f.c);
        assert!(!cpu.registers.f.n);
        assert!(!cpu.registers.f.h);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn ccf_toggles_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.f.c = false;
        ccf(&mut cpu);
        assert!(cpu.registers.f.c);
        ccf(&mut cpu);
        assert!(!cpu.registers.f.c);
    }

    #[test]
    fn ccf_clears_n_and_h() {
        let mut cpu = Cpu::new();
        cpu.registers.f.n = true;
        cpu.registers.f.h = true;
        ccf(&mut cpu);
        assert!(!cpu.registers.f.n);
        assert!(!cpu.registers.f.h);
    }
}
