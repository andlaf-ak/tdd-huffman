use tdd_huffman::BitStream;

#[test]
fn write_single_bit_to_stream() {
    let mut output = Vec::new();
    let mut bit_stream = BitStream::new(&mut output);

    // Write a single 0 bit
    bit_stream.write_bit(0).unwrap();

    // Write a single 1 bit
    bit_stream.write_bit(1).unwrap();

    // No bytes should be emitted yet (only 2 bits)
    assert_eq!(output, vec![]);
}

#[test]
fn write_multiple_individual_bits_in_sequence() {
    let mut output = Vec::new();
    let mut bit_stream = BitStream::new(&mut output);

    // Write a sequence of bits: 1, 0, 1, 1, 0
    bit_stream.write_bit(1).unwrap();
    bit_stream.write_bit(0).unwrap();
    bit_stream.write_bit(1).unwrap();
    bit_stream.write_bit(1).unwrap();
    bit_stream.write_bit(0).unwrap();

    // No bytes should be emitted yet (only 5 bits)
    assert_eq!(output, vec![]);
}

#[test]
fn write_exactly_8_bits_and_verify_byte_emission() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Write exactly 8 bits: 1, 0, 1, 1, 0, 0, 1, 0 (represents binary 10110010 = 178)
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
    }

    // Byte should have been emitted to the output stream
    assert_eq!(output, vec![178u8]);
}

#[test]
fn write_more_than_8_bits_and_verify_multiple_byte_emissions() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Write 17 bits total across multiple bytes
        // First byte: 1, 0, 1, 1, 0, 0, 1, 0 (binary 10110010 = 178)
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();

        // Second byte: 1, 1, 0, 1, 0, 1, 1, 1 (binary 11010111 = 215)
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();

        // One additional bit: 1 (should not emit yet, needs 7 more bits)
        bit_stream.write_bit(1).unwrap();
    }

    // Two complete bytes should have been emitted
    assert_eq!(output, vec![178u8, 215u8]);
}

#[test]
fn write_exactly_16_bits_and_verify_two_complete_bytes() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Write exactly 16 bits (2 complete bytes)
        // First byte: 1, 0, 1, 1, 0, 0, 1, 0 (binary 10110010 = 178)
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();

        // Second byte: 0, 0, 1, 1, 1, 1, 0, 1 (binary 00111101 = 61)
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();

        // At this point, we should have emitted exactly 2 bytes and have no pending bits
        // This test verifies that the bit stream correctly handles complete byte boundaries
    }

    // Exactly two bytes should have been emitted
    assert_eq!(output, vec![178u8, 61u8]);
}

#[test]
fn flush_incomplete_byte_with_padding() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Write only 5 bits: 1, 0, 1, 1, 0
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(1).unwrap();
        bit_stream.write_bit(0).unwrap();

        // Flush should emit the incomplete byte with zero-padding
        // Expected: 10110000 (5 bits + 3 zero-padded bits) = 176
        bit_stream.flush().unwrap();
    }

    // One byte should have been emitted with zero-padding
    assert_eq!(output, vec![176u8]);
}
