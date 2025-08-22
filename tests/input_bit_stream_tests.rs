use tdd_huffman::InputBitStream;

#[test]
fn read_single_bit_from_stream_containing_one_byte() {
    // Create a stream with one byte: 10110010 (178)
    let data = vec![178u8];
    let mut input_stream = InputBitStream::new(&data[..]);

    // Read the first bit (should be 1)
    let bit = input_stream.read_bit().unwrap();
    assert_eq!(bit, 1);
}

#[test]
fn read_multiple_individual_bits_in_sequence() {
    // Create a stream with one byte: 10110010 (178)
    let data = vec![178u8];
    let mut input_stream = InputBitStream::new(&data[..]);

    // Read all 8 bits in sequence and verify each one
    assert_eq!(input_stream.read_bit().unwrap(), 1); // MSB
    assert_eq!(input_stream.read_bit().unwrap(), 0);
    assert_eq!(input_stream.read_bit().unwrap(), 1);
    assert_eq!(input_stream.read_bit().unwrap(), 1);
    assert_eq!(input_stream.read_bit().unwrap(), 0);
    assert_eq!(input_stream.read_bit().unwrap(), 0);
    assert_eq!(input_stream.read_bit().unwrap(), 1);
    assert_eq!(input_stream.read_bit().unwrap(), 0); // LSB
}

#[test]
fn read_bits_across_multiple_bytes() {
    // Create a stream with two bytes: [10110010, 11010000]
    let data = vec![178u8, 208u8]; // 178 = 10110010, 208 = 11010000
    let mut input_stream = InputBitStream::new(&data[..]);

    // Read first 8 bits (from first byte)
    assert_eq!(input_stream.read_bit().unwrap(), 1);
    assert_eq!(input_stream.read_bit().unwrap(), 0);
    assert_eq!(input_stream.read_bit().unwrap(), 1);
    assert_eq!(input_stream.read_bit().unwrap(), 1);
    assert_eq!(input_stream.read_bit().unwrap(), 0);
    assert_eq!(input_stream.read_bit().unwrap(), 0);
    assert_eq!(input_stream.read_bit().unwrap(), 1);
    assert_eq!(input_stream.read_bit().unwrap(), 0);

    // Read next 3 bits (from second byte: 110...)
    assert_eq!(input_stream.read_bit().unwrap(), 1);
    assert_eq!(input_stream.read_bit().unwrap(), 1);
    assert_eq!(input_stream.read_bit().unwrap(), 0);
}
