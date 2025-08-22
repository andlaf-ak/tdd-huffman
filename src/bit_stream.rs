pub struct BitStream {
    bit_count: usize,
    bits: Vec<u8>,
}

impl Default for BitStream {
    fn default() -> Self {
        Self::new()
    }
}

impl BitStream {
    pub fn new() -> Self {
        Self { 
            bit_count: 0,
            bits: Vec::new(),
        }
    }

    pub fn write_bit(&mut self, bit: u8) {
        debug_assert!(bit <= 1, "Bit value must be 0 or 1, got: {bit}");
        self.bits.push(bit);
        self.bit_count += 1;
    }

    pub fn bit_count(&self) -> usize {
        self.bit_count
    }

    pub fn get_bits(&self) -> Vec<u8> {
        self.bits.clone()
    }
}
