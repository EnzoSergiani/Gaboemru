#![cfg(test)]

use crate::common::types::Byte;

pub fn build_rom(title: &str, cartridge_type: Byte, rom_size: Byte, ram_size: Byte) -> Vec<Byte> {
    let mut rom = vec![0u8; 0x150];
    let title_bytes = title.as_bytes();
    rom[0x0134..0x0134 + title_bytes.len()].copy_from_slice(title_bytes);
    rom[0x0147] = cartridge_type;
    rom[0x0148] = rom_size;
    rom[0x0149] = ram_size;
    let checksum =
        (0x0134..=0x014C).fold(0u8, |acc, addr| acc.wrapping_sub(rom[addr]).wrapping_sub(1));
    rom[0x014D] = checksum;
    rom
}
