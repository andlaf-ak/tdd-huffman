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

#[test]
fn deserializes_two_leaf_tree_structure() {
    // Arrange: bit stream 0 1{A} 1{B} = [binary: 0 1 01000001 1 01000010]
    // = 0101000001101000010 (19 bits total)
    // Packed into bytes: 01010000 01101000 01000000 = 0x50, 0x68, 0x40
    let bit_stream_data = vec![0x50, 0x68, 0x40]; 
    let cursor = Cursor::new(bit_stream_data);
    let mut bit_stream = InputBitStream::new(cursor);
    
    // Act
    let result = deserialize_tree(&mut bit_stream).expect("Deserialization should succeed");
    
    // Assert: Should be an internal node with two leaf children
    assert!(result.symbol().is_none()); // Internal node has no symbol
    assert_eq!(result.frequency(), 2); // Combined frequency of children
    
    // Check left child (should be leaf A)
    let left_child = result.left_child().expect("Internal node should have left child");
    assert_eq!(left_child.symbol(), Some(b'A'));
    assert_eq!(left_child.frequency(), 1);
    
    // Check right child (should be leaf B)
    let right_child = result.right_child().expect("Internal node should have right child");
    assert_eq!(right_child.symbol(), Some(b'B'));
    assert_eq!(right_child.frequency(), 1);
}