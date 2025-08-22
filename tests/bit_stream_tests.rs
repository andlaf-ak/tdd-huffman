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
