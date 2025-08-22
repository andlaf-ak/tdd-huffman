use crate::constants::{BITS_PER_BYTE, MSB_MASK};
use std::io::Read;

pub struct InputBitStream<R> {
    reader: R,
    current_byte: u8,
    bits_in_current_byte: usize,
}

impl<R: Read> InputBitStream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_byte: 0,
            bits_in_current_byte: 0,
        }
    }

    fn load_next_byte(&mut self) -> std::io::Result<()> {
        let mut buffer = [0u8; 1];
        self.reader.read_exact(&mut buffer)?;
        self.current_byte = buffer[0];
        self.bits_in_current_byte = BITS_PER_BYTE;
        Ok(())
    }

    pub fn read_bit(&mut self) -> std::io::Result<u8> {
        // If no bits remaining in current byte, read next byte
        if self.bits_in_current_byte == 0 {
            self.load_next_byte()?;
        }

        // Extract the leftmost bit using MSB mask
        let bit = if self.current_byte & MSB_MASK != 0 {
            1
        } else {
            0
        };

        // Shift left to move to next bit position
        self.current_byte <<= 1;
        self.bits_in_current_byte -= 1;

        Ok(bit)
    }
}
