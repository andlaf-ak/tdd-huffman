use crate::constants::{BITS_PER_BYTE, MSB_MASK};
use std::io::Read;

/// A bit-level input stream that reads individual bits from a byte-oriented reader.
///
/// This struct wraps any type implementing `std::io::Read` and provides bit-level
/// access to the underlying data. Bits are read from left to right (MSB to LSB)
/// within each byte, consistent with the OutputBitStream's bit ordering.
pub struct InputBitStream<R> {
    reader: R,
    current_byte: u8,
    bits_in_current_byte: usize,
}

impl<R: Read> InputBitStream<R> {
    /// Creates a new InputBitStream wrapping the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_byte: 0,
            bits_in_current_byte: 0,
        }
    }

    /// Loads the next byte from the underlying reader into the current_byte buffer.
    /// Resets the bit counter to indicate a full byte (8 bits) is available.
    fn load_next_byte(&mut self) -> std::io::Result<()> {
        let mut buffer = [0u8; 1];
        self.reader.read_exact(&mut buffer)?;
        self.current_byte = buffer[0];
        self.bits_in_current_byte = BITS_PER_BYTE;
        Ok(())
    }

    /// Reads a single bit from the stream.
    ///
    /// Bits are read from left to right (MSB to LSB) within each byte.
    /// When a byte is exhausted, the next byte is automatically loaded.
    ///
    /// Returns 0 or 1, or an I/O error if no more data is available.
    pub fn read_bit(&mut self) -> std::io::Result<u8> {
        debug_assert!(
            self.bits_in_current_byte <= BITS_PER_BYTE,
            "Invalid bit count state: {} bits remaining in current byte",
            self.bits_in_current_byte
        );

        // If no bits remaining in current byte, load the next byte
        if self.bits_in_current_byte == 0 {
            self.load_next_byte()?;
        }

        // Extract the leftmost bit using MSB mask
        let bit = if self.current_byte & MSB_MASK != 0 {
            1
        } else {
            0
        };

        // Advance to the next bit position by shifting left and decrementing counter
        self.current_byte <<= 1;
        self.bits_in_current_byte -= 1;

        Ok(bit)
    }
}
