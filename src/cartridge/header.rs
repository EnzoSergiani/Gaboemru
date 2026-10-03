use tracing::error;

use crate::common::types::Byte;

#[derive(Debug, PartialEq)]
pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
    Unknown(Byte),
}

impl CartridgeType {
    fn from_byte(byte: Byte) -> Self {
        match byte {
            0x00 => CartridgeType::RomOnly,
            0x01 => CartridgeType::Mbc1,
            0x02 => CartridgeType::Mbc1Ram,
            0x03 => CartridgeType::Mbc1RamBattery,
            0x05 => CartridgeType::Mbc2,
            0x06 => CartridgeType::Mbc2Battery,
            0x08 => CartridgeType::RomRam,
            0x09 => CartridgeType::RomRamBattery,
            0x0B => CartridgeType::Mmm01,
            0x0C => CartridgeType::Mmm01Ram,
            0x0D => CartridgeType::Mmm01RamBattery,
            0x0F => CartridgeType::Mbc3TimerBattery,
            0x10 => CartridgeType::Mbc3TimerRamBattery,
            0x11 => CartridgeType::Mbc3,
            0x12 => CartridgeType::Mbc3Ram,
            0x13 => CartridgeType::Mbc3RamBattery,
            0x19 => CartridgeType::Mbc5,
            0x1A => CartridgeType::Mbc5Ram,
            0x1B => CartridgeType::Mbc5RamBattery,
            0x1C => CartridgeType::Mbc5Rumble,
            0x1D => CartridgeType::Mbc5RumbleRam,
            0x1E => CartridgeType::Mbc5RumbleRamBattery,
            0x20 => CartridgeType::Mbc6,
            0x22 => CartridgeType::Mbc7SensorRumbleRamBattery,
            0xFC => CartridgeType::PocketCamera,
            0xFD => CartridgeType::BandaiTama5,
            0xFE => CartridgeType::HuC3,
            0xFF => CartridgeType::HuC1RamBattery,
            other => CartridgeType::Unknown(other),
        }
    }
}

pub struct CartridgeHeader {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub rom_size_bytes: usize,
    pub ram_size_bytes: usize,
}

impl CartridgeHeader {
    pub fn parse(rom: &[Byte]) -> Self {
        let cartridge_header = Self {
            title: parse_title(rom),
            cartridge_type: CartridgeType::from_byte(rom[0x0147]),
            rom_size_bytes: rom_size_from_byte(rom[0x0148]),
            ram_size_bytes: ram_size_from_byte(rom[0x0149]),
        };

        if !is_checksum_valid(rom) {
            error!("Header checksum incorrect");
            panic!("Header check checksum incorrect");
        }

        cartridge_header
    }
}

fn parse_title(rom: &[Byte]) -> String {
    let bytes = &rom[0x0134..=0x0143];
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

fn rom_size_from_byte(byte: Byte) -> usize {
    match byte {
        0x00 => 32 * 1024,
        0x01 => 64 * 1024,
        0x02 => 128 * 1024,
        0x03 => 256 * 1024,
        0x04 => 512 * 1024,
        0x05 => 1024 * 1024,
        0x06 => 2 * 1024 * 1024,
        0x07 => 4 * 1024 * 1024,
        0x08 => 8 * 1024 * 1024,
        0x52 => 1152 * 1024,
        0x53 => 1280 * 1024,
        0x54 => 1536 * 1024,
        _ => 0,
    }
}

fn ram_size_from_byte(byte: Byte) -> usize {
    match byte {
        0x00 => 0,
        0x01 => 2 * 1024,
        0x02 => 8 * 1024,
        0x03 => 32 * 1024,
        0x04 => 128 * 1024,
        0x05 => 64 * 1024,
        _ => 0,
    }
}

fn is_checksum_valid(rom: &[Byte]) -> bool {
    let calculated = (0x0134..=0x014C).fold(0u8, |checksum, address| {
        checksum.wrapping_sub(rom[address]).wrapping_sub(1)
    });

    calculated == rom[0x014D]
}

#[cfg(test)]
mod tests {
    use crate::common::test_helpers::build_rom;

    use super::*;

    #[test]
    fn parses_title() {
        let rom = build_rom("TETRIS", 0x00, 0x00, 0x00);
        let header = CartridgeHeader::parse(&rom);
        assert_eq!(header.title, "TETRIS");
    }

    #[test]
    fn parses_cartridge_type() {
        let cases = [
            (0x00, CartridgeType::RomOnly),
            (0x01, CartridgeType::Mbc1),
            (0x02, CartridgeType::Mbc1Ram),
            (0x03, CartridgeType::Mbc1RamBattery),
            (0x05, CartridgeType::Mbc2),
            (0x06, CartridgeType::Mbc2Battery),
            (0x08, CartridgeType::RomRam),
            (0x09, CartridgeType::RomRamBattery),
            (0x0B, CartridgeType::Mmm01),
            (0x0C, CartridgeType::Mmm01Ram),
            (0x0D, CartridgeType::Mmm01RamBattery),
            (0x0F, CartridgeType::Mbc3TimerBattery),
            (0x10, CartridgeType::Mbc3TimerRamBattery),
            (0x11, CartridgeType::Mbc3),
            (0x12, CartridgeType::Mbc3Ram),
            (0x13, CartridgeType::Mbc3RamBattery),
            (0x19, CartridgeType::Mbc5),
            (0x1A, CartridgeType::Mbc5Ram),
            (0x1B, CartridgeType::Mbc5RamBattery),
            (0x1C, CartridgeType::Mbc5Rumble),
            (0x1D, CartridgeType::Mbc5RumbleRam),
            (0x1E, CartridgeType::Mbc5RumbleRamBattery),
            (0x20, CartridgeType::Mbc6),
            (0x22, CartridgeType::Mbc7SensorRumbleRamBattery),
            (0xFC, CartridgeType::PocketCamera),
            (0xFD, CartridgeType::BandaiTama5),
            (0xFE, CartridgeType::HuC3),
            (0xFF, CartridgeType::HuC1RamBattery),
        ];

        for (code, expected) in cases {
            let rom = build_rom("", code, 0x00, 0x00);
            let header = CartridgeHeader::parse(&rom);

            assert_eq!(header.cartridge_type, expected);
        }
    }

    #[test]
    fn parses_rom_size() {
        let rom = build_rom("", 0x00, 0x02, 0x00);
        let header = CartridgeHeader::parse(&rom);
        assert_eq!(header.rom_size_bytes, 128 * 1024);
    }

    #[test]
    fn parses_ram_size() {
        let rom = build_rom("", 0x00, 0x00, 0x03);
        let header = CartridgeHeader::parse(&rom);
        assert_eq!(header.ram_size_bytes, 32 * 1024);
    }

    #[test]
    fn parses_checksum() {
        let rom = build_rom("", 0x00, 0x00, 0x00);
        let is_valid = is_checksum_valid(&rom);
        assert_eq!(is_valid, true);
    }

    #[test]
    #[should_panic(expected = "checksum")]
    fn rejects_invalid_checksum() {
        let mut rom = build_rom("", 0x00, 0x00, 0x00);
        rom[0x014D] = 0x00;
        CartridgeHeader::parse(&rom);
    }
}
