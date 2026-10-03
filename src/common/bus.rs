use crate::common::types::{Address, Byte};

pub trait Bus {
    fn read(&self, address: Address) -> Byte;
    fn write(&mut self, address: Address, value: Byte);
}
