pub struct BitStream {
    bit_count: usize,
}

impl Default for BitStream {
    fn default() -> Self {
        Self::new()
    }
}

impl BitStream {
    pub fn new() -> Self {
        Self { bit_count: 0 }
    }

    pub fn write_bit(&mut self, bit: u8) {
        debug_assert!(bit <= 1, "Bit value must be 0 or 1, got: {bit}");
        self.bit_count += 1;
    }

    pub fn bit_count(&self) -> usize {
        self.bit_count
    }
}
