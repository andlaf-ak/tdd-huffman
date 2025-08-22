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
