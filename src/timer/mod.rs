pub struct Timer {}

impl Timer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
