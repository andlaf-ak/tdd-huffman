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
