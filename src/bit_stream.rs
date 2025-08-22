pub struct BitStream {
    bits: Vec<u8>,
}

impl Default for BitStream {
    fn default() -> Self {
        Self::new()
    }
}

impl BitStream {
    pub fn new() -> Self {
        Self { bits: Vec::new() }
    }

    pub fn write_bit(&mut self, bit: u8) {
        debug_assert!(bit <= 1, "Bit value must be 0 or 1, got: {bit}");
        self.bits.push(bit);
    }

    pub fn bit_count(&self) -> usize {
        self.bits.len()
    }

    pub fn get_bits(&self) -> &[u8] {
        &self.bits
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for chunk in self.bits.chunks(8) {
            if chunk.len() == 8 {
                let mut byte_value = 0u8;
                for (i, &bit) in chunk.iter().enumerate() {
                    if bit == 1 {
                        byte_value |= 1 << (7 - i);
                    }
                }
                bytes.push(byte_value);
            }
        }
        bytes
    }

    pub fn byte_count(&self) -> usize {
        self.bits.len() / 8
    }
}
