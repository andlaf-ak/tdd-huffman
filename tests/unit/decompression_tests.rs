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
    
    // Original input was 10 bytes long
    let original_length = 10;
    
    // Act
    let result = decompress(&tree, &mut bit_stream, original_length);
    
    // Assert: Should produce 10 A's
    let expected = vec![b'A'; 10];
    assert_eq!(result, expected);
}
