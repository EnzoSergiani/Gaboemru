use super::mbc::Mbc;
use crate::common::types::{Address, Byte};

pub struct RomOnly {
    rom: Vec<Byte>,
}

impl RomOnly {
    pub fn new(rom: Vec<Byte>) -> Self {
        Self { rom }
    }
}

impl Mbc for RomOnly {
    fn read_rom(&self, address: Address) -> Byte {
        self.rom.get(address as usize).copied().unwrap_or(0xFF)
    }
    fn write_rom(&mut self, _addr: Address, _value: Byte) {}
    fn read_ram(&self, _address: Address) -> Byte {
        0xFF
    }
    fn write_ram(&mut self, _address: Address, _value: Byte) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_rom() {
        let rom = vec![0xAA, 0xBB, 0xCC];
        let mbc = RomOnly::new(rom);
        assert_eq!(mbc.read_rom(0x0000), 0xAA);
        assert_eq!(mbc.read_rom(0x0002), 0xCC);
    }

    #[test]
    fn write_rom() {
        let rom = vec![0xAA, 0xBB, 0xCC];
        let mut mbc = RomOnly::new(rom);
        mbc.write_rom(0x0000, 0xDD);
        mbc.write_rom(0x0002, 0xFF);
        assert_eq!(mbc.read_rom(0x0000), 0xAA);
        assert_eq!(mbc.read_rom(0x0002), 0xCC);
    }

    #[test]
    fn read_ram() {
        let rom = vec![0xAA, 0xBB, 0xCC];
        let mbc = RomOnly::new(rom);
        assert_eq!(mbc.read_ram(0x0000), 0xFF);
        assert_eq!(mbc.read_ram(0x0002), 0xFF);
    }

    #[test]
    fn write_ram() {
        let rom = vec![0xAA, 0xBB, 0xCC];
        let mut mbc = RomOnly::new(rom);
        mbc.write_ram(0x0000, 0xDD);
        mbc.write_ram(0x0002, 0xFF);
        assert_eq!(mbc.read_ram(0x0000), 0xFF);
        assert_eq!(mbc.read_ram(0x0002), 0xFF);
    }
}
