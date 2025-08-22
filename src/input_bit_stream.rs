use std::io::Read;

pub struct InputBitStream<R> {
    reader: R,
    current_byte: u8,
    bits_remaining_in_current_byte: usize,
}

const BITS_PER_BYTE: usize = 8;

impl<R: Read> InputBitStream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_byte: 0,
            bits_remaining_in_current_byte: 0,
        }
    }

    pub fn read_bit(&mut self) -> std::io::Result<u8> {
        // If no bits remaining in current byte, read next byte
        if self.bits_remaining_in_current_byte == 0 {
            let mut buffer = [0u8; 1];
            self.reader.read_exact(&mut buffer)?;
            self.current_byte = buffer[0];
            self.bits_remaining_in_current_byte = BITS_PER_BYTE;
        }

        // Extract the leftmost bit
        let bit = if self.current_byte & 0x80 != 0 { 1 } else { 0 };

        // Shift left to move to next bit position
        self.current_byte <<= 1;
        self.bits_remaining_in_current_byte -= 1;

        Ok(bit)
    }
}
