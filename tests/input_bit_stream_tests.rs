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
