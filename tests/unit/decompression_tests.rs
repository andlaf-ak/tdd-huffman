use std::io::Cursor;
use tdd_huffman::{decompress, HuffmanNode, InputBitStream};

#[test]
fn decompresses_ten_zeros_to_ten_as_with_single_node_tree() {
    // Arrange: Tree with single node for 'A' 
    let tree = HuffmanNode::new_leaf(b'A', 10);
    
    // Bit stream with 10 zeros (representing 10 'A's in single-node tree)
    let compressed_data = vec![0x00, 0x00]; // 10 zero bits: 0000000000 + 6 padding bits
    let cursor = Cursor::new(compressed_data);
    let mut bit_stream = InputBitStream::new(cursor);
    
    // Output stream to write decompressed data
    let mut output = Vec::new();
    
    // Original input was 10 bytes long
    let original_length = 10;
    
    // Act
    decompress(&tree, &mut bit_stream, &mut output, original_length)
        .expect("Decompression should succeed");
    
    // Assert: Should produce 10 A's
    let expected = vec![b'A'; 10];
    assert_eq!(output, expected);
}

#[test]
fn decompresses_bits_to_ab_sequence_with_two_node_tree() {
    // Arrange: Tree with two nodes - A (left, code 0) and B (right, code 1)
    let left_leaf = HuffmanNode::new_leaf(b'A', 1);
    let right_leaf = HuffmanNode::new_leaf(b'B', 1);
    let tree = HuffmanNode::new_internal(left_leaf, right_leaf);
    
    // Bit stream "0110001011101000" = 0x62E8 (16 bits)
    let compressed_data = vec![0x62, 0xE8]; // 01100010 11101000
    let cursor = Cursor::new(compressed_data);
    let mut bit_stream = InputBitStream::new(cursor);
    
    // Output stream to write decompressed data
    let mut output = Vec::new();
    
    // Original input was 13 bytes long
    let original_length = 13;
    
    // Act
    decompress(&tree, &mut bit_stream, &mut output, original_length)
        .expect("Decompression should succeed");
    
    // Assert: Should produce "ABBAAABABBBAB"
    let expected = b"ABBAAABABBBAB".to_vec();
    assert_eq!(output, expected);
}
