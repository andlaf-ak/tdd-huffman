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
    assert!(
        code == "0" || code == "1",
        "Single symbol should have 1-bit code, got: {}",
        code
    );
}

#[test]
fn two_nodes_generate_complementary_codes() {
    // This test covers a simple binary tree with two symbols
    // The tree structure should be:
    //       root
    //      /    \
    //   A(65)   B(66)
    // Expected codes: A="0", B="1" (or A="1", B="0")

    let left_leaf = HuffmanNode::new_leaf(65u8, 3usize); // 'A' appears 3 times
    let right_leaf = HuffmanNode::new_leaf(66u8, 5usize); // 'B' appears 5 times
    let tree_root = HuffmanNode::new_internal(left_leaf, right_leaf);

    let codes = extract_huffman_codes(&tree_root);

    // Should have exactly two entries
    assert_eq!(codes.len(), 2);

    // Both symbols should be present
    assert!(codes.contains_key(&65u8)); // 'A'
    assert!(codes.contains_key(&66u8)); // 'B'

    let code_a = codes.get(&65u8).unwrap();
    let code_b = codes.get(&66u8).unwrap();

    // Both codes should be exactly 1 bit
    assert_eq!(code_a.len(), 1);
    assert_eq!(code_b.len(), 1);

    // Codes should be complementary (one "0", one "1")
    assert!(
        (code_a == "0" && code_b == "1") || (code_a == "1" && code_b == "0"),
        "Expected complementary codes, got A='{}' and B='{}'",
        code_a,
        code_b
    );
}
