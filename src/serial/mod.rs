use crate::common::types::Byte;

pub struct Serial {
    sb: Byte,
}

impl Serial {
    pub fn new() -> Self {
        Self { sb: 0x00 }
    }

    pub fn sb(&self) -> Byte {
        self.sb
    }

    pub fn set_sb(&mut self, value: Byte) {
        self.sb = value;
    }
}

impl Default for Serial {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sb_roundtrips_through_setter() {
        let mut serial = Serial::new();
        serial.set_sb(0x42);
        assert_eq!(serial.sb(), 0x42);
    }
}
