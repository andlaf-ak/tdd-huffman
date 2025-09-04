use crate::constants::{BITS_PER_BYTE, MSB_MASK};
use std::io::Read;

pub struct InputBitStream<R> {
    reader: R,
    current_byte: u8,
    bits_in_current_byte: usize,
}

impl<R: Read> InputBitStream<R> {
    // Creates a new bit stream reader from any byte-oriented reader
    // Initializes with no current byte loaded and no bits available
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_byte: 0,
            bits_in_current_byte: 0,
        }
    }

    // Loads the next byte from the underlying reader
    // Reads exactly one byte and sets up bit tracking for that byte
    // Resets the bit counter to 8, indicating 8 bits are now available
    fn load_next_byte(&mut self) -> std::io::Result<()> {
        let mut buffer = [0u8; 1];
        self.reader.read_exact(&mut buffer)?;
        self.current_byte = buffer[0];
        self.bits_in_current_byte = BITS_PER_BYTE;
        Ok(())
    }

    // Reads a single bit from the stream
    // If no bits are available in current byte, loads the next byte first
    // Extracts the most significant bit from current_byte using bitwise AND with 128
    // Shifts the byte left to position the next bit as the most significant
    // Returns the extracted bit as 0 or 1
    pub fn read_bit(&mut self) -> std::io::Result<u8> {
        debug_assert!(
            self.bits_in_current_byte <= BITS_PER_BYTE,
            "Invalid bit count state: {} bits remaining in current byte",
            self.bits_in_current_byte
        );

        if self.bits_in_current_byte == 0 {
            self.load_next_byte()?;
        }

        let bit = if self.current_byte & MSB_MASK != 0 {
            1
        } else {
            0
        };

        self.current_byte <<= 1;
        self.bits_in_current_byte -= 1;

        Ok(bit)
    }
}
