use tdd_huffman::tree_deserialization::deserialize_tree;
use tdd_huffman::input_bit_stream::InputBitStream;
use std::io::Cursor;

#[test]
fn deserializes_single_leaf_tree_with_symbol_a() {
    // Arrange: bit stream 1{A} = [binary: 1 01000001] = [hex: 0xA0, 0x80] 
    // 1 01000001 needs to be split across bytes: 10100000 10000000
    let bit_stream_data = vec![0xA0, 0x80]; // 10100000 10000000 in binary  
    let cursor = Cursor::new(bit_stream_data);
    let mut bit_stream = InputBitStream::new(cursor);
    
    // Act
    let result = deserialize_tree(&mut bit_stream).expect("Deserialization should succeed");
    
    // Assert
    assert_eq!(result.symbol(), Some(b'A'));
    assert_eq!(result.frequency(), 1);
    assert_eq!(result.as_leaf(), Some((b'A', 1)));
}