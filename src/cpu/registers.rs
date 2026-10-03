use crate::common::types::{Byte, Word};

#[derive(Copy, Clone)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl Flags {
    pub fn to_byte(self) -> Byte {
        (self.z as Byte) << 7
            | (self.n as Byte) << 6
            | (self.h as Byte) << 5
            | (self.c as Byte) << 4
    }

    pub fn from_byte(byte: Byte) -> Self {
        Self {
            z: byte & 0x80 != 0,
            n: byte & 0x40 != 0,
            h: byte & 0x20 != 0,
            c: byte & 0x10 != 0,
        }
    }
}

pub struct Registers {
    pub a: Byte,
    pub f: Flags,
    pub b: Byte,
    pub c: Byte,
    pub d: Byte,
    pub e: Byte,
    pub h: Byte,
    pub l: Byte,
    pub sp: Word,
    pub pc: Word,
}

impl Registers {
    pub fn af(&self) -> Word {
        (self.a as Word) << 8 | self.f.to_byte() as Word
    }
    pub fn set_af(&mut self, value: Word) {
        self.a = (value >> 8) as Byte;
        self.f = Flags::from_byte(value as Byte);
    }

    pub fn bc(&self) -> Word {
        (self.b as Word) << 8 | self.c as Word
    }
    pub fn set_bc(&mut self, value: Word) {
        self.b = (value >> 8) as Byte;
        self.c = value as Byte;
    }

    pub fn de(&self) -> Word {
        (self.d as Word) << 8 | self.e as Word
    }
    pub fn set_de(&mut self, value: Word) {
        self.d = (value >> 8) as Byte;
        self.e = value as Byte;
    }

    pub fn hl(&self) -> Word {
        (self.h as Word) << 8 | self.l as Word
    }
    pub fn set_hl(&mut self, value: Word) {
        self.h = (value >> 8) as Byte;
        self.l = value as Byte;
    }

    pub fn default() -> Self {
        Registers {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn af_packs_a_and_f() {
        let mut regs = Registers::default();
        regs.a = 0x12;
        regs.f = Flags::from_byte(0x80);
        assert_eq!(regs.af(), 0x1280);
    }

    #[test]
    fn set_af_unpacks_a_and_f() {
        let mut regs = Registers::default();
        regs.set_af(0x1280);
        assert_eq!(regs.a, 0x12);
        assert!(regs.f.z);
        assert!(!regs.f.n);
    }

    #[test]
    fn bc_roundtrip() {
        let mut regs = Registers::default();
        regs.set_bc(0xABCD);
        assert_eq!(regs.bc(), 0xABCD);
        assert_eq!(regs.b, 0xAB);
        assert_eq!(regs.c, 0xCD);
    }

    #[test]
    fn de_roundtrip() {
        let mut regs = Registers::default();
        regs.set_de(0xABCD);
        assert_eq!(regs.de(), 0xABCD);
        assert_eq!(regs.d, 0xAB);
        assert_eq!(regs.e, 0xCD);
    }

    #[test]
    fn hl_roundtrip() {
        let mut regs = Registers::default();
        regs.set_hl(0xABCD);
        assert_eq!(regs.hl(), 0xABCD);
        assert_eq!(regs.h, 0xAB);
        assert_eq!(regs.l, 0xCD);
    }

    #[test]
    fn flags_to_byte_sets_correct_bits() {
        let flags = Flags {
            z: true,
            n: false,
            h: true,
            c: false,
        };
        assert_eq!(flags.to_byte(), 0b1010_0000);
    }

    #[test]
    fn flags_from_byte_ignores_lower_nibble() {
        let flags = Flags::from_byte(0b1111_1111);
        assert_eq!(flags.to_byte(), 0b1111_0000);
    }

    #[test]
    fn flags_roundtrip() {
        for byte in [0x00, 0x80, 0x40, 0x20, 0x10, 0xF0] {
            let flags = Flags::from_byte(byte);
            assert_eq!(flags.to_byte(), byte);
        }
    }
}
