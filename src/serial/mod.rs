pub struct Serial {}

impl Serial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Serial {
    fn default() -> Self {
        Self::new()
    }
}
