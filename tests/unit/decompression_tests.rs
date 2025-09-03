use std::io::Cursor;
use tdd_huffman::{decompress, HuffmanNode, OutputBitStream, serialize_tree_to_bits};

#[test]
fn decompresses_ten_zeros_to_ten_as_with_single_node_tree() {
    // Arrange: Tree with single node for 'A' 
    let tree = HuffmanNode::new_leaf(b'A', 10);
    
    // Create properly formatted compressed data with header and serialized tree
    let mut compressed_data = Vec::new();
    
    // Write the original data length as a 4-byte little-endian header
    let original_length = 10u32;
    compressed_data.extend_from_slice(&original_length.to_le_bytes());
    
    // Serialize the tree to the compressed data
    let mut bit_stream = OutputBitStream::new(&mut compressed_data);
    serialize_tree_to_bits(&tree, &mut bit_stream).expect("Tree serialization should succeed");
    
    // For single-node tree, no additional bits are needed for the data itself
    // (the tree structure alone determines the output)
    bit_stream.flush().expect("Should flush successfully");
    
    // Create input reader from compressed data
    let cursor = Cursor::new(compressed_data);
    
    // Output stream to write decompressed data
    let mut output = Vec::new();
    
    // Act
    decompress(cursor, &mut output)
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
    
    // Create properly formatted compressed data with header and serialized tree
    let mut compressed_data = Vec::new();
    
    // Write the original data length as a 4-byte little-endian header
    let original_length = 13u32;
    compressed_data.extend_from_slice(&original_length.to_le_bytes());
    
    // Serialize the tree to the compressed data
    let mut bit_stream = OutputBitStream::new(&mut compressed_data);
    serialize_tree_to_bits(&tree, &mut bit_stream).expect("Tree serialization should succeed");
    
    // Add the encoded data: "0110001011101000" = 0x62E8 (16 bits)
    // This represents "ABBAAABABBBAB" with A=0, B=1
    bit_stream.write_bit(0).unwrap(); // A
    bit_stream.write_bit(1).unwrap(); // B
    bit_stream.write_bit(1).unwrap(); // B
    bit_stream.write_bit(0).unwrap(); // A
    bit_stream.write_bit(0).unwrap(); // A
    bit_stream.write_bit(0).unwrap(); // A
    bit_stream.write_bit(1).unwrap(); // B
    bit_stream.write_bit(0).unwrap(); // A
    bit_stream.write_bit(1).unwrap(); // B
    bit_stream.write_bit(1).unwrap(); // B
    bit_stream.write_bit(1).unwrap(); // B
    bit_stream.write_bit(0).unwrap(); // A
    bit_stream.write_bit(1).unwrap(); // B
    
    bit_stream.flush().expect("Should flush successfully");
    
    // Create input reader from compressed data
    let cursor = Cursor::new(compressed_data);
    
    // Output stream to write decompressed data
    let mut output = Vec::new();
    
    // Act
    decompress(cursor, &mut output)
        .expect("Decompression should succeed");
    
    // Assert: Should produce "ABBAAABABBBAB"
    let expected = b"ABBAAABABBBAB".to_vec();
    assert_eq!(output, expected);
}
