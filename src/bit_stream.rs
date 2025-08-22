pub struct BitStream<W> {
    writer: W,
    current_byte: u8,
    bits_in_current_byte: usize,
}

impl<W: std::io::Write> BitStream<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            current_byte: 0,
            bits_in_current_byte: 0,
        }
    }

    pub fn write_bit(&mut self, bit: u8) -> std::io::Result<()> {
        debug_assert!(bit <= 1, "Bit value must be 0 or 1, got: {bit}");

        // Set the bit in the current byte (from left to right)
        if bit == 1 {
            self.current_byte |= 1 << (7 - self.bits_in_current_byte);
        }

        self.bits_in_current_byte += 1;

        // Emit byte to output stream when we have 8 bits, then reset
        if self.bits_in_current_byte == 8 {
            self.writer.write_all(&[self.current_byte])?;
            self.current_byte = 0;
            self.bits_in_current_byte = 0;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        // If we have any pending bits, emit them as a padded byte
        if self.bits_in_current_byte > 0 {
            // The remaining bits are already in the correct positions (left-aligned)
            // Zero bits are automatically in the right positions since we start with 0
            self.writer.write_all(&[self.current_byte])?;
            self.current_byte = 0;
            self.bits_in_current_byte = 0;
        }
        Ok(())
    }
}
