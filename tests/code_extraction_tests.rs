use tdd_huffman::{extract_huffman_codes, HuffmanNode};

#[test]
fn single_node_generates_single_bit_code() {
    // This test covers the edge case where we have only one symbol
    // In this case, the tree is just a single leaf node
    // The convention for single-symbol Huffman codes is to use "0" or "1"
    // (since we need at least 1 bit per symbol)
    
    let single_node = HuffmanNode::new_leaf(65u8, 10usize); // 'A' appears 10 times
    
    let codes = extract_huffman_codes(&single_node);
    
    // Should have exactly one entry
    assert_eq!(codes.len(), 1);
    
    // Should map symbol 'A' (65) to a single-bit code
    assert!(codes.contains_key(&65u8));
    let code = codes.get(&65u8).unwrap();
    
    // Code should be exactly 1 bit (either "0" or "1")
    assert_eq!(code.len(), 1);
    assert!(code == "0" || code == "1", "Single symbol should have 1-bit code, got: {}", code);
}
