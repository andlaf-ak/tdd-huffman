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

#[test]
fn write_seven_plus_one_bits_emits_one_byte() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Write 7 bits: 1,0,1,1,0,1,0
        for bit in [1, 0, 1, 1, 0, 1, 0] {
            bit_stream.write_bit(bit).unwrap();
        }

        // Write 8th bit: 1 → should trigger emission
        bit_stream.write_bit(1).unwrap();
    }

    // Should emit: 10110101 = 181
    assert_eq!(output, vec![181u8]);
}

#[test]
fn write_nine_bits_emits_one_byte_buffers_one_bit() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Write 9 bits: first 8 should emit immediately, 9th should buffer
        // Bits: 1,0,1,1,0,1,0,1 (first byte) + 1 (buffered)
        for bit in [1, 0, 1, 1, 0, 1, 0, 1, 1] {
            bit_stream.write_bit(bit).unwrap();
        }
    }

    // Only first 8 bits emitted: 10110101 = 181
    assert_eq!(output, vec![181u8]);
}

#[test]
fn write_bits_in_various_combinations() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Test 3+5 = 8 bits total
        // First 3 bits: 1,0,1
        for bit in [1, 0, 1] {
            bit_stream.write_bit(bit).unwrap();
        }

        // Next 5 bits: 1,0,1,0,1 → completes the byte
        for bit in [1, 0, 1, 0, 1] {
            bit_stream.write_bit(bit).unwrap();
        }
    }

    // Should emit: 10110101 = 181
    assert_eq!(output, vec![181u8]);
}

#[test]
fn flush_after_exactly_eight_bits_emits_no_additional_bytes() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Write exactly 8 bits: 1,0,1,1,0,1,0,1
        for bit in [1, 0, 1, 1, 0, 1, 0, 1] {
            bit_stream.write_bit(bit).unwrap();
        }

        // Flush should not emit anything additional
        bit_stream.flush().unwrap();
    }

    // Should have exactly 1 byte: 10110101 = 181
    assert_eq!(output, vec![181u8]);
}

#[test]
fn write_nine_bits_then_flush_emits_two_bytes() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Write 9 bits: 1,0,1,1,0,1,0,1 (first byte) + 1 (buffered)
        for bit in [1, 0, 1, 1, 0, 1, 0, 1, 1] {
            bit_stream.write_bit(bit).unwrap();
        }

        // Flush should emit the buffered bit with padding
        bit_stream.flush().unwrap();
    }

    // Should have 2 bytes: 181 (first byte) + 128 (10000000 = padded second byte)
    assert_eq!(output, vec![181u8, 128u8]);
}

#[test]
fn flush_empty_stream_emits_nothing() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);
        bit_stream.flush().unwrap();
    }

    assert_eq!(output, vec![]);
}

#[test]
fn multiple_consecutive_flushes_emit_no_extra_bytes() {
    let mut output = Vec::new();
    {
        let mut bit_stream = BitStream::new(&mut output);

        // Write 3 bits: 1,0,1
        for bit in [1, 0, 1] {
            bit_stream.write_bit(bit).unwrap();
        }

        // Multiple flushes should only emit once
        bit_stream.flush().unwrap();
        bit_stream.flush().unwrap();
        bit_stream.flush().unwrap();
    }

    // Should emit exactly one padded byte: 10100000 = 160
    assert_eq!(output, vec![160u8]);
}
