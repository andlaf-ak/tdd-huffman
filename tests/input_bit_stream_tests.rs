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

#[test]
fn read_seven_bits_from_one_byte_verify_one_bit_remains() {
    // Create a stream with one byte: 10110010 (178)
    const BYTE_VALUE: u8 = 178; // 10110010 in binary
    let data = vec![BYTE_VALUE];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Read first 7 bits: 1011001
    let first_seven_bits = [1, 0, 1, 1, 0, 0, 1];
    assert_bits_read_in_sequence(&mut input_stream, &first_seven_bits);

    // Verify the last bit (8th bit) is still available: 0
    let last_bit = input_stream.read_bit().unwrap();
    assert_eq!(last_bit, 0);
}

#[test]
fn read_nine_bits_from_two_bytes_verify_remaining_bits() {
    // Create a stream with two bytes: [10110010, 11010000]
    const FIRST_BYTE: u8 = 178; // 10110010 in binary
    const SECOND_BYTE: u8 = 208; // 11010000 in binary
    let data = vec![FIRST_BYTE, SECOND_BYTE];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Read 9 bits: first 8 bits (10110010) + 1 bit from second byte (1)
    let nine_bits = [1, 0, 1, 1, 0, 0, 1, 0, 1];
    assert_bits_read_in_sequence(&mut input_stream, &nine_bits);

    // Verify remaining 7 bits from second byte: 1010000
    let remaining_bits = [1, 0, 1, 0, 0, 0, 0];
    assert_bits_read_in_sequence(&mut input_stream, &remaining_bits);
}

#[test]
fn read_all_bits_from_multi_byte_input() {
    // Create a stream with 3 bytes: [11110000, 10101010, 00001111]
    const BYTE1: u8 = 240; // 11110000 in binary
    const BYTE2: u8 = 170; // 10101010 in binary
    const BYTE3: u8 = 15; // 00001111 in binary
    let data = vec![BYTE1, BYTE2, BYTE3];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Read all 24 bits
    let all_bits = [
        // First byte: 11110000
        1, 1, 1, 1, 0, 0, 0, 0, // Second byte: 10101010
        1, 0, 1, 0, 1, 0, 1, 0, // Third byte: 00001111
        0, 0, 0, 0, 1, 1, 1, 1,
    ];
    assert_bits_read_in_sequence(&mut input_stream, &all_bits);
}

#[test]
fn handle_empty_input_stream() {
    // Create an empty stream
    let data: Vec<u8> = vec![];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Attempt to read a bit should return an error
    let result = input_stream.read_bit();
    assert!(result.is_err());
}

#[test]
fn attempt_to_read_beyond_available_bits() {
    // Create a stream with one byte: 10110010 (178)
    const BYTE_VALUE: u8 = 178; // 10110010 in binary
    let data = vec![BYTE_VALUE];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Read all 8 bits successfully
    let all_eight_bits = [1, 0, 1, 1, 0, 0, 1, 0];
    assert_bits_read_in_sequence(&mut input_stream, &all_eight_bits);

    // Attempt to read one more bit should return an error
    let result = input_stream.read_bit();
    assert!(result.is_err());
}

#[test]
fn read_bits_in_various_chunk_sizes_from_same_input() {
    // Create a stream with two bytes: [11110000, 10101010]
    const BYTE1: u8 = 240; // 11110000 in binary
    const BYTE2: u8 = 170; // 10101010 in binary
    let data = vec![BYTE1, BYTE2];

    // Test reading the same input in different chunk patterns

    // Pattern 1: Read 1, then 3, then 4, then 8 bits
    let mut input_stream1 = create_bit_stream_from_bytes(&data);

    // Read 1 bit: 1
    assert_eq!(input_stream1.read_bit().unwrap(), 1);

    // Read 3 bits: 111
    let three_bits = [1, 1, 1];
    assert_bits_read_in_sequence(&mut input_stream1, &three_bits);

    // Read 4 bits: 0000
    let four_bits = [0, 0, 0, 0];
    assert_bits_read_in_sequence(&mut input_stream1, &four_bits);

    // Read remaining 8 bits: 10101010
    let eight_bits = [1, 0, 1, 0, 1, 0, 1, 0];
    assert_bits_read_in_sequence(&mut input_stream1, &eight_bits);

    // Pattern 2: Read 5, then 11 bits from the same data
    let mut input_stream2 = create_bit_stream_from_bytes(&data);

    // Read 5 bits: 11110
    let five_bits = [1, 1, 1, 1, 0];
    assert_bits_read_in_sequence(&mut input_stream2, &five_bits);

    // Read remaining 11 bits: 00010101010
    let eleven_bits = [0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    assert_bits_read_in_sequence(&mut input_stream2, &eleven_bits);
}

#[test]
fn read_from_stream_with_zero_padded_final_byte() {
    // Simulate output from OutputBitStream that wrote 9 bits then flushed
    // This would produce: [complete_byte, partial_byte_with_padding]
    // Example: 9 bits "101100101" -> [10110010, 10000000]
    //                    ^--8 bits--^  ^1 bit + 7 padding zeros

    const COMPLETE_BYTE: u8 = 178; // 10110010 - first 8 bits
    const PADDED_BYTE: u8 = 128; // 10000000 - 1 meaningful bit + 7 padding zeros
    let data = vec![COMPLETE_BYTE, PADDED_BYTE];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Read the first 8 bits (complete byte)
    let first_eight_bits = [1, 0, 1, 1, 0, 0, 1, 0];
    assert_bits_read_in_sequence(&mut input_stream, &first_eight_bits);

    // Read the meaningful bit from the padded byte
    assert_eq!(input_stream.read_bit().unwrap(), 1);

    // The remaining 7 bits are padding zeros (but our reader doesn't know that)
    // They would be read as actual zeros
    let padding_bits = [0, 0, 0, 0, 0, 0, 0];
    assert_bits_read_in_sequence(&mut input_stream, &padding_bits);
}

#[test]
fn read_from_stream_with_exact_bit_count() {
    // This test demonstrates reading exactly the meaningful bits
    // In a real implementation, we might need to track the actual bit count separately
    // For now, we test reading exactly 9 meaningful bits from our padded stream

    const COMPLETE_BYTE: u8 = 178; // 10110010 - first 8 bits
    const PADDED_BYTE: u8 = 128; // 10000000 - 1 meaningful bit + 7 padding zeros
    let data = vec![COMPLETE_BYTE, PADDED_BYTE];
    let mut input_stream = create_bit_stream_from_bytes(&data);

    // Read exactly 9 meaningful bits: 101100101
    let nine_meaningful_bits = [1, 0, 1, 1, 0, 0, 1, 0, 1];
    assert_bits_read_in_sequence(&mut input_stream, &nine_meaningful_bits);

    // Note: In a real compression scenario, we'd stop here based on metadata
    // The remaining 7 bits are padding and shouldn't be read as meaningful data
}
