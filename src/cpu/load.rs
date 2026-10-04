use crate::common::types::{Address, Byte, Cycles, Word};

use super::{Bus, Cpu};

macro_rules! ld_r8_r8 {
    ($name:ident, $dest:ident, $src:ident) => {
        #[allow(clippy::self_assignment)]
        pub fn $name(cpu: &mut Cpu) -> Cycles {
            cpu.registers.$dest = cpu.registers.$src;
            4
        }
    };
}

ld_r8_r8!(ld_a_a, a, a);
ld_r8_r8!(ld_a_b, a, b);
ld_r8_r8!(ld_a_c, a, c);
ld_r8_r8!(ld_a_d, a, d);
ld_r8_r8!(ld_a_e, a, e);
ld_r8_r8!(ld_a_h, a, h);
ld_r8_r8!(ld_a_l, a, l);
ld_r8_r8!(ld_b_a, b, a);
ld_r8_r8!(ld_b_b, b, b);
ld_r8_r8!(ld_b_c, b, c);
ld_r8_r8!(ld_b_d, b, d);
ld_r8_r8!(ld_b_e, b, e);
ld_r8_r8!(ld_b_h, b, h);
ld_r8_r8!(ld_b_l, b, l);
ld_r8_r8!(ld_c_a, c, a);
ld_r8_r8!(ld_c_b, c, b);
ld_r8_r8!(ld_c_c, c, c);
ld_r8_r8!(ld_c_d, c, d);
ld_r8_r8!(ld_c_e, c, e);
ld_r8_r8!(ld_c_h, c, h);
ld_r8_r8!(ld_c_l, c, l);
ld_r8_r8!(ld_d_a, d, a);
ld_r8_r8!(ld_d_b, d, b);
ld_r8_r8!(ld_d_c, d, c);
ld_r8_r8!(ld_d_d, d, d);
ld_r8_r8!(ld_d_e, d, e);
ld_r8_r8!(ld_d_h, d, h);
ld_r8_r8!(ld_d_l, d, l);
ld_r8_r8!(ld_e_a, e, a);
ld_r8_r8!(ld_e_b, e, b);
ld_r8_r8!(ld_e_c, e, c);
ld_r8_r8!(ld_e_d, e, d);
ld_r8_r8!(ld_e_e, e, e);
ld_r8_r8!(ld_e_h, e, h);
ld_r8_r8!(ld_e_l, e, l);
ld_r8_r8!(ld_h_a, h, a);
ld_r8_r8!(ld_h_b, h, b);
ld_r8_r8!(ld_h_c, h, c);
ld_r8_r8!(ld_h_d, h, d);
ld_r8_r8!(ld_h_e, h, e);
ld_r8_r8!(ld_h_h, h, h);
ld_r8_r8!(ld_h_l, h, l);
ld_r8_r8!(ld_l_a, l, a);
ld_r8_r8!(ld_l_b, l, b);
ld_r8_r8!(ld_l_c, l, c);
ld_r8_r8!(ld_l_d, l, d);
ld_r8_r8!(ld_l_e, l, e);
ld_r8_r8!(ld_l_h, l, h);
ld_r8_r8!(ld_l_l, l, l);

macro_rules! ld_r8_n8 {
    ($name:ident, $dest:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let value: Byte = cpu.fetch_byte(bus);
            cpu.registers.$dest = value;
            8
        }
    };
}

ld_r8_n8!(ld_a_n8, a);
ld_r8_n8!(ld_b_n8, b);
ld_r8_n8!(ld_c_n8, c);
ld_r8_n8!(ld_d_n8, d);
ld_r8_n8!(ld_e_n8, e);
ld_r8_n8!(ld_h_n8, h);
ld_r8_n8!(ld_l_n8, l);

macro_rules! ld_r16_n16 {
    ($name:ident, $setter:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let low: Byte = cpu.fetch_byte(bus);
            let high: Byte = cpu.fetch_byte(bus);
            let word: Word = (high as Word) << 8 | low as Word;
            cpu.registers.$setter(word);
            12
        }
    };
}

ld_r16_n16!(ld_bc_n16, set_bc);
ld_r16_n16!(ld_de_n16, set_de);
ld_r16_n16!(ld_hl_n16, set_hl);

pub fn ld_sp_n16<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let low: Byte = cpu.fetch_byte(bus);
    let high: Byte = cpu.fetch_byte(bus);
    cpu.registers.sp = (high as Word) << 8 | low as Word;
    12
}

macro_rules! ld_hl_r8 {
    ($name:ident, $src:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let value: Byte = cpu.registers.$src;
            bus.write(cpu.registers.hl(), value);
            8
        }
    };
}

ld_hl_r8!(ld_hl_a, a);
ld_hl_r8!(ld_hl_b, b);
ld_hl_r8!(ld_hl_c, c);
ld_hl_r8!(ld_hl_d, d);
ld_hl_r8!(ld_hl_e, e);
ld_hl_r8!(ld_hl_h, h);
ld_hl_r8!(ld_hl_l, l);

pub fn ld_hl_n8<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let value: Byte = cpu.fetch_byte(bus);
    bus.write(cpu.registers.hl(), value);
    12
}

macro_rules! ld_r8_hl {
    ($name:ident, $dest:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            cpu.registers.$dest = bus.read(cpu.registers.hl());
            8
        }
    };
}

ld_r8_hl!(ld_a_hl, a);
ld_r8_hl!(ld_b_hl, b);
ld_r8_hl!(ld_c_hl, c);
ld_r8_hl!(ld_d_hl, d);
ld_r8_hl!(ld_e_hl, e);
ld_r8_hl!(ld_h_hl, h);
ld_r8_hl!(ld_l_hl, l);

macro_rules! ld_r16_a {
    ($name:ident, $dest:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let value: Byte = cpu.registers.a;
            bus.write(cpu.registers.$dest(), value);
            8
        }
    };
}

ld_r16_a!(ld_bc_a, bc);
ld_r16_a!(ld_de_a, de);

macro_rules! ld_a_r16 {
    ($name:ident, $src:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let value: Byte = bus.read(cpu.registers.$src());
            cpu.registers.a = value;
            8
        }
    };
}

ld_a_r16!(ld_a_bc, bc);
ld_a_r16!(ld_a_de, de);

pub fn ld_hli_a<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.registers.hl();
    bus.write(address, cpu.registers.a);
    cpu.registers.set_hl(address.wrapping_add(1));
    8
}

pub fn ld_hld_a<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.registers.hl();
    bus.write(address, cpu.registers.a);
    cpu.registers.set_hl(address.wrapping_sub(1));
    8
}

pub fn ld_a_hli<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.registers.hl();
    cpu.registers.a = bus.read(address);
    cpu.registers.set_hl(address.wrapping_add(1));
    8
}

pub fn ld_a_hld<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.registers.hl();
    cpu.registers.a = bus.read(address);
    cpu.registers.set_hl(address.wrapping_sub(1));
    8
}

pub fn ld_n16_a<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.fetch_word(bus);
    bus.write(address, cpu.registers.a);
    16
}

pub fn ld_a_n16<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.fetch_word(bus);
    cpu.registers.a = bus.read(address);
    16
}

pub fn ldh_n8_a<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let offset = cpu.fetch_byte(bus);
    let address: Address = 0xFF00 | offset as Word;
    bus.write(address, cpu.registers.a);
    12
}

pub fn ldh_a_n8<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let offset = cpu.fetch_byte(bus);
    let address: Address = 0xFF00 | offset as Word;
    cpu.registers.a = bus.read(address);
    12
}

pub fn ldh_c_a<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = 0xFF00 | cpu.registers.c as Word;
    bus.write(address, cpu.registers.a);
    8
}

pub fn ldh_a_c<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = 0xFF00 | cpu.registers.c as Word;
    cpu.registers.a = bus.read(address);
    8
}

pub fn ld_sp_hl(cpu: &mut Cpu) -> Cycles {
    cpu.registers.sp = cpu.registers.hl();
    8
}

pub fn ld_hl_sp_e8<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let offset = cpu.fetch_byte(bus) as i8 as i16;
    let sp = cpu.registers.sp as i16;
    let result: Word = sp.wrapping_add(offset) as Word;

    let half_carry: bool = (cpu.registers.sp & 0x0F) + (offset as Word & 0x0F) > 0x0F;
    let carry: bool = (cpu.registers.sp & 0xFF) + (offset as Word & 0xFF) > 0xFF;

    cpu.registers.f.z = false;
    cpu.registers.f.n = false;
    cpu.registers.f.h = half_carry;
    cpu.registers.f.c = carry;

    cpu.registers.set_hl(result);
    12
}

pub fn ld_n16_sp<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
    let address: Address = cpu.fetch_word(bus);
    bus.write(address, cpu.registers.sp as Byte);
    bus.write(address.wrapping_add(1), (cpu.registers.sp >> 8) as Byte);
    20
}

macro_rules! push_r16 {
    ($name:ident, $getter:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let value: Word = cpu.registers.$getter();
            cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
            bus.write(cpu.registers.sp, (value >> 8) as Byte);
            cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
            bus.write(cpu.registers.sp, value as Byte);
            16
        }
    };
}

push_r16!(push_bc, bc);
push_r16!(push_de, de);
push_r16!(push_hl, hl);
push_r16!(push_af, af);

macro_rules! pop_r16 {
    ($name:ident, $setter:ident) => {
        pub fn $name<B: Bus>(cpu: &mut Cpu, bus: &mut B) -> Cycles {
            let low: Byte = bus.read(cpu.registers.sp);
            cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
            let high: Byte = bus.read(cpu.registers.sp);
            cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
            let value: Word = (high as Word) << 8 | low as Word;
            cpu.registers.$setter(value);
            12
        }
    };
}

pop_r16!(pop_bc, set_bc);
pop_r16!(pop_de, set_de);
pop_r16!(pop_hl, set_hl);
pop_r16!(pop_af, set_af);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_helpers::FlatRam;
    use crate::cpu::Cpu;

    #[test]
    fn ld_r8_r8_copies_value() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x42;
        assert_eq!(ld_b_a(&mut cpu), 4);
        assert_eq!(cpu.registers.b, 0x42);
    }

    #[test]
    fn ld_r8_n8_reads_immediate() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        bus.write(cpu.registers.pc, 0x77);
        assert_eq!(ld_a_n8(&mut cpu, &mut bus), 8);
        assert_eq!(cpu.registers.a, 0x77);
    }

    #[test]
    fn ld_r16_n16_reads_little_endian() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        bus.write(cpu.registers.pc, 0xCD);
        bus.write(cpu.registers.pc + 1, 0xAB);
        assert_eq!(ld_bc_n16(&mut cpu, &mut bus), 12);
        assert_eq!(cpu.registers.bc(), 0xABCD);
    }

    #[test]
    fn ld_hli_a_writes_and_increments() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.set_hl(0xC000);
        cpu.registers.a = 0x42;
        assert_eq!(ld_hli_a(&mut cpu, &mut bus), 8);
        assert_eq!(bus.read(0xC000), 0x42);
        assert_eq!(cpu.registers.hl(), 0xC001);
    }

    #[test]
    fn ld_hld_a_writes_and_decrements() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.set_hl(0xC000);
        cpu.registers.a = 0x42;
        ld_hld_a(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.hl(), 0xBFFF);
    }

    #[test]
    fn ldh_a_c_reads_from_high_page() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.c = 0x05;
        bus.write(0xFF05, 0x99);
        assert_eq!(ldh_a_c(&mut cpu, &mut bus), 8);
        assert_eq!(cpu.registers.a, 0x99);
    }

    #[test]
    fn push_pop_bc_roundtrip() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.set_bc(0x1234);
        let sp_before = cpu.registers.sp;
        assert_eq!(push_bc(&mut cpu, &mut bus), 16);
        assert_eq!(cpu.registers.sp, sp_before.wrapping_sub(2));
        cpu.registers.set_bc(0x0000);
        assert_eq!(pop_bc(&mut cpu, &mut bus), 12);
        assert_eq!(cpu.registers.bc(), 0x1234);
        assert_eq!(cpu.registers.sp, sp_before);
    }

    #[test]
    fn pop_af_masks_lower_nibble() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        bus.write(cpu.registers.sp, 0xFF);
        bus.write(cpu.registers.sp + 1, 0x12);
        pop_af(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.a, 0x12);
        assert_eq!(cpu.registers.f.to_byte(), 0xF0);
    }

    #[test]
    fn ld_hl_sp_e8_positive_offset() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.sp = 0xFFF8;
        bus.write(cpu.registers.pc, 0x02);
        ld_hl_sp_e8(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.hl(), 0xFFFA);
    }

    #[test]
    fn ld_hl_sp_e8_negative_offset() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.sp = 0xFFF8;
        bus.write(cpu.registers.pc, 0xFE);
        ld_hl_sp_e8(&mut cpu, &mut bus);
        assert_eq!(cpu.registers.hl(), 0xFFF6);
    }

    #[test]
    fn ld_n16_sp_writes_little_endian() {
        let mut cpu = Cpu::new();
        let mut bus = FlatRam::new();
        cpu.registers.sp = 0xABCD;
        bus.write(cpu.registers.pc, 0x00);
        bus.write(cpu.registers.pc + 1, 0xC0);
        assert_eq!(ld_n16_sp(&mut cpu, &mut bus), 20);
        assert_eq!(bus.read(0xC000), 0xCD);
        assert_eq!(bus.read(0xC001), 0xAB);
    }
}
