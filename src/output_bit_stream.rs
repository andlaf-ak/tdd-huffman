use crate::constants::BITS_PER_BYTE;

pub struct OutputBitStream<W> {
    writer: W,
    current_byte: u8,
    bits_in_current_byte: usize,
}

impl<W: std::io::Write> OutputBitStream<W> {
    // Creates a new bit stream writer that wraps any byte-oriented writer
    // Initializes with an empty current byte and zero bits accumulated
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            current_byte: 0,
            bits_in_current_byte: 0,
        }
    }

    // Writes the current accumulated byte to the underlying writer
    // Resets the current byte to 0 and bit counter to 0
    // Called automatically when 8 bits have been accumulated
    fn emit_current_byte(&mut self) -> std::io::Result<()> {
        self.writer.write_all(&[self.current_byte])?;
        self.current_byte = 0;
        self.bits_in_current_byte = 0;
        Ok(())
    }

    // Writes a single bit (0 or 1) to the stream
    // Accumulates bits into a byte buffer: sets the bit at the correct position
    // When 8 bits are accumulated, automatically writes the complete byte
    // Uses bitwise OR to set the bit at the calculated position within the byte
    pub fn write_bit(&mut self, bit: u8) -> std::io::Result<()> {
        debug_assert!(bit <= 1, "Bit value must be 0 or 1, got: {bit}");

        if bit == 1 {
            self.current_byte |= 1 << (BITS_PER_BYTE - 1 - self.bits_in_current_byte);
        }

        self.bits_in_current_byte += 1;

        if self.bits_in_current_byte == BITS_PER_BYTE {
            self.emit_current_byte()?;
        }

        Ok(())
    }

    // Writes any remaining bits as a final byte
    // If there are leftover bits (1-7), they are padded with zeros to complete the byte
    // Must be called at the end to ensure all bits are written to the output
    pub fn flush(&mut self) -> std::io::Result<()> {
        if self.bits_in_current_byte > 0 {
            self.emit_current_byte()?;
        }
        Ok(())
    }
}
