use tdd_huffman::BitStream;

#[test]
fn write_single_bit_to_stream() {
    let mut bit_stream = BitStream::new();

    // Write a single 0 bit
    bit_stream.write_bit(0);

    // Verify the bit was written (we'll need some way to check the internal state)
    assert_eq!(bit_stream.bit_count(), 1);

    // Write a single 1 bit
    bit_stream.write_bit(1);

    // Verify we now have 2 bits written
    assert_eq!(bit_stream.bit_count(), 2);
}

#[test]
fn write_multiple_individual_bits_in_sequence() {
    let mut bit_stream = BitStream::new();

    // Write a sequence of bits: 1, 0, 1, 1, 0
    bit_stream.write_bit(1);
    bit_stream.write_bit(0);
    bit_stream.write_bit(1);
    bit_stream.write_bit(1);
    bit_stream.write_bit(0);

    // Verify the correct number of bits were written
    assert_eq!(bit_stream.bit_count(), 5);

    // We should be able to retrieve the sequence of bits written
    assert_eq!(bit_stream.get_bits(), vec![1, 0, 1, 1, 0]);
}

#[test]
fn write_exactly_8_bits_and_verify_byte_emission() {
    let mut bit_stream = BitStream::new();

    // Write exactly 8 bits: 1, 0, 1, 1, 0, 0, 1, 0 (represents binary 10110010 = 178)
    bit_stream.write_bit(1);
    bit_stream.write_bit(0);
    bit_stream.write_bit(1);
    bit_stream.write_bit(1);
    bit_stream.write_bit(0);
    bit_stream.write_bit(0);
    bit_stream.write_bit(1);
    bit_stream.write_bit(0);

    // Verify we have 8 bits
    assert_eq!(bit_stream.bit_count(), 8);

    // Should be able to get the bits as a complete byte
    assert_eq!(bit_stream.get_bytes(), vec![178u8]);

    // Should have exactly 1 complete byte
    assert_eq!(bit_stream.byte_count(), 1);
}
