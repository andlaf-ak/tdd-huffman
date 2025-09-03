use tdd_huffman::{serialize_tree, HuffmanNode};

#[test]
fn single_leaf_tree_serializes_to_1_followed_by_symbol() {
    let tree = HuffmanNode::new_leaf(65u8, 5);

    let result = serialize_tree(&tree);

    assert_eq!(result, "101000001");
}

#[test]
fn tree_with_two_leaves_serializes_correctly() {
    let left_leaf = HuffmanNode::new_leaf(65u8, 3);
    let right_leaf = HuffmanNode::new_leaf(66u8, 5);
    let tree = HuffmanNode::new_internal(left_leaf, right_leaf);

    let result = serialize_tree(&tree);

    assert_eq!(result, "0101000001101000010");
}

#[test]
fn tree_with_three_nodes_serializes_correctly() {
    let leaf_a = HuffmanNode::new_leaf(65u8, 1); // 'A'
    let leaf_b = HuffmanNode::new_leaf(66u8, 2); // 'B'
    let leaf_c = HuffmanNode::new_leaf(67u8, 3); // 'C'

    let internal_bc = HuffmanNode::new_internal(leaf_b, leaf_c);

    let tree = HuffmanNode::new_internal(leaf_a, internal_bc);

    let result = serialize_tree(&tree);

    assert_eq!(result, "01010000010101000010101000011");
}

#[test]
fn complex_four_node_tree_serializes_correctly() {
    let leaf_a = HuffmanNode::new_leaf(65u8, 1); // 'A'
    let leaf_b = HuffmanNode::new_leaf(66u8, 2); // 'B'
    let leaf_c = HuffmanNode::new_leaf(67u8, 3); // 'C'
    let leaf_d = HuffmanNode::new_leaf(68u8, 4); // 'D'

    let node2 = HuffmanNode::new_internal(leaf_b, leaf_c);

    let node1 = HuffmanNode::new_internal(node2, leaf_d);

    let tree = HuffmanNode::new_internal(leaf_a, node1);

    let result = serialize_tree(&tree);

    assert_eq!(result, "010100000100101000010101000011101000100");
}
