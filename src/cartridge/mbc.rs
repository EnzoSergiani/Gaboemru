use crate::common::types::{Address, Byte};

pub trait Mbc {
    fn read_rom(&self, address: Address) -> Byte;
    fn write_rom(&mut self, address: Address, value: Byte);
    fn read_ram(&self, address: Address) -> Byte;
    fn write_ram(&mut self, address: Address, value: Byte);
}
