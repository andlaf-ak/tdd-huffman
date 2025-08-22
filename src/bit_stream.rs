pub struct BitStream {
    bit_count: usize,
}

impl BitStream {
    pub fn new() -> Self {
        Self { bit_count: 0 }
    }

    pub fn write_bit(&mut self, _bit: u8) {
        self.bit_count += 1;
    }

    pub fn bit_count(&self) -> usize {
        self.bit_count
    }
}
