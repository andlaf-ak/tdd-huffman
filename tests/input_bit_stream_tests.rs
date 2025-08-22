use tdd_huffman::InputBitStream;

// Helper function to read and verify a sequence of bits
fn assert_bits_read_in_sequence(input_stream: &mut InputBitStream<&[u8]>, expected_bits: &[u8]) {
    for &expected_bit in expected_bits {
        let actual_bit = input_stream.read_bit().unwrap();
        assert_eq!(actual_bit, expected_bit);
    }
}

// Helper function to create a bit stream from byte data
fn create_bit_stream_from_bytes(data: &[u8]) -> InputBitStream<&[u8]> {
    InputBitStream::new(data)
}

#[test]
fn read_single_bit_from_stream_containing_one_byte() {
    // Create a stream with one byte: 10110010 (178)
    const BYTE_VALUE: u8 = 178; // 10110010 in binary
    let data = vec![BYTE_VALUE];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Read the first bit (should be 1)
    let bit = input_stream.read_bit().unwrap();
    assert_eq!(bit, 1);
}

#[test]
fn read_multiple_individual_bits_in_sequence() {
    // Create a stream with one byte: 10110010 (178)
    const BYTE_VALUE: u8 = 178; // 10110010 in binary
    let data = vec![BYTE_VALUE];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Expected bits from 10110010 (MSB to LSB)
    let expected_bits = [1, 0, 1, 1, 0, 0, 1, 0];
    assert_bits_read_in_sequence(&mut input_stream, &expected_bits);
}

#[test]
fn read_bits_across_multiple_bytes() {
    // Create a stream with two bytes: [10110010, 11010000]
    const FIRST_BYTE: u8 = 178; // 10110010 in binary
    const SECOND_BYTE: u8 = 208; // 11010000 in binary
    let data = vec![FIRST_BYTE, SECOND_BYTE];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Read first 8 bits (from first byte: 10110010)
    let first_byte_bits = [1, 0, 1, 1, 0, 0, 1, 0];
    assert_bits_read_in_sequence(&mut input_stream, &first_byte_bits);

    // Read next 3 bits (from second byte: 110...)
    let partial_second_byte_bits = [1, 1, 0];
    assert_bits_read_in_sequence(&mut input_stream, &partial_second_byte_bits);
}
