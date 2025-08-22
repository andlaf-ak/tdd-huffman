pub struct BitStream<W> {
    writer: W,
    current_byte: u8,
    bits_in_current_byte: usize,
}

const BITS_PER_BYTE: usize = 8;

impl<W: std::io::Write> BitStream<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            current_byte: 0,
            bits_in_current_byte: 0,
        }
    }

    fn emit_current_byte(&mut self) -> std::io::Result<()> {
        self.writer.write_all(&[self.current_byte])?;
        self.current_byte = 0;
        self.bits_in_current_byte = 0;
        Ok(())
    }

    pub fn write_bit(&mut self, bit: u8) -> std::io::Result<()> {
        debug_assert!(bit <= 1, "Bit value must be 0 or 1, got: {bit}");

        // Set the bit in the current byte (from left to right)
        if bit == 1 {
            self.current_byte |= 1 << (BITS_PER_BYTE - 1 - self.bits_in_current_byte);
        }

        self.bits_in_current_byte += 1;

        // Emit byte to output stream when we have 8 bits, then reset
        if self.bits_in_current_byte == BITS_PER_BYTE {
            self.emit_current_byte()?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        // If we have any pending bits, emit them as a padded byte
        if self.bits_in_current_byte > 0 {
            self.emit_current_byte()?;
        }
        Ok(())
    }
}
