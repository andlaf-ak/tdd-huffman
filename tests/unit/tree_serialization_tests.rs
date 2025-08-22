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

#[test]
fn tree_with_two_leaves_serializes_correctly() {
    // Arrange: Create a tree with root and two leaves
    // Left leaf: symbol 65 (ASCII 'A'), Right leaf: symbol 66 (ASCII 'B')
    let left_leaf = HuffmanNode::new_leaf(65u8, 3);
    let right_leaf = HuffmanNode::new_leaf(66u8, 5);
    let tree = HuffmanNode::new_internal(left_leaf, right_leaf);

    // Act: Serialize the tree
    let result = serialize_tree(&tree);

    // Assert: Should be "0" + "1" + left_symbol + "1" + right_symbol
    // = "0" + "1" + "01000001" + "1" + "01000010" = "0101000001101000010"
    assert_eq!(result, "0101000001101000010");
}

#[test]
fn tree_with_three_nodes_serializes_correctly() {
    // Arrange: Create a three-node tree structure
    // Root with A (65) on left, and internal node (B-C) on right
    let leaf_a = HuffmanNode::new_leaf(65u8, 1); // 'A'
    let leaf_b = HuffmanNode::new_leaf(66u8, 2); // 'B'
    let leaf_c = HuffmanNode::new_leaf(67u8, 3); // 'C'
    
    // Create internal node with B and C
    let internal_bc = HuffmanNode::new_internal(leaf_b, leaf_c);
    
    // Create root with A on left, BC internal node on right
    let tree = HuffmanNode::new_internal(leaf_a, internal_bc);

    // Act: Serialize the tree
    let result = serialize_tree(&tree);

    // Assert: Should be "0" + "1{A}" + "0" + "1{B}" + "1{C}"
    // = "0" + "101000001" + "0" + "101000010" + "101000011"
    // = "01010000010101000010101000011"
    assert_eq!(result, "01010000010101000010101000011");
}

#[test]
fn complex_four_node_tree_serializes_correctly() {
    // Arrange: Create complex tree structure
    // Root → Left: A, Right: (Node1 → Left: (Node2 → Left: B, Right: C), Right: D)
    let leaf_a = HuffmanNode::new_leaf(65u8, 1); // 'A'
    let leaf_b = HuffmanNode::new_leaf(66u8, 2); // 'B'
    let leaf_c = HuffmanNode::new_leaf(67u8, 3); // 'C'
    let leaf_d = HuffmanNode::new_leaf(68u8, 4); // 'D'

    // Create Node2 with B and C
    let node2 = HuffmanNode::new_internal(leaf_b, leaf_c);

    // Create Node1 with Node2 and D
    let node1 = HuffmanNode::new_internal(node2, leaf_d);

    // Create root with A and Node1
    let tree = HuffmanNode::new_internal(leaf_a, node1);

    // Act: Serialize the tree
    let result = serialize_tree(&tree);

    // Assert: Should be "0" + "1{A}" + "0" + "0" + "1{B}" + "1{C}" + "1{D}"
    // = "0" + "101000001" + "0" + "0" + "101000010" + "101000011" + "101000100"
    // = "010100000100101000010101000011101000100"
    assert_eq!(result, "010100000100101000010101000011101000100");
}
