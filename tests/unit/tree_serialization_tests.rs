use tdd_huffman::{serialize_tree, HuffmanNode};

#[test]
fn single_leaf_tree_serializes_to_1_followed_by_symbol() {
    // Arrange: Create a single leaf tree with symbol 65 (ASCII 'A')
    let tree = HuffmanNode::new_leaf(65u8, 5);

    // Act: Serialize the tree
    let result = serialize_tree(&tree);

    // Assert: Should be "1" + 8-bit representation of 65 = "101000001"
    assert_eq!(result, "101000001");
}
