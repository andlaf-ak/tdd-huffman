use tdd_huffman::{merge_leaf_nodes, merge_with_leaf_node};

#[test]
fn merge_two_leaf_nodes() {
    let left_node = (65u8, 3usize); // 'A' with frequency 3
    let right_node = (66u8, 2usize); // 'B' with frequency 2

    let merged_node = merge_leaf_nodes(left_node, right_node);

    // The merged node should have combined frequency (3 + 2 = 5)
    // and should contain references to both child nodes
    assert_eq!(merged_node.frequency(), 5);
    assert_eq!(merged_node.left_child(), Some(&left_node));
    assert_eq!(merged_node.right_child(), Some(&right_node));
    // Internal nodes should not have a symbol value
    assert!(merged_node.symbol().is_none());
}

#[test]
fn merge_leaf_node_with_non_leaf_node() {
    // First create an internal node from two leaf nodes
    let leaf_a = (65u8, 3usize); // 'A' with frequency 3
    let leaf_b = (66u8, 2usize); // 'B' with frequency 2
    let internal_node = merge_leaf_nodes(leaf_a, leaf_b); // Internal node with frequency 5

    // Now merge this internal node with another leaf node
    let leaf_c = (67u8, 4usize); // 'C' with frequency 4
    let merged_node = merge_with_leaf_node(internal_node, leaf_c);

    // The new merged node should have combined frequency (5 + 4 = 9)
    assert_eq!(merged_node.frequency(), 9);
    // Should be an internal node (no symbol)
    assert!(merged_node.symbol().is_none());
    // Should have the internal node as left child and leaf as right child
    assert_eq!(merged_node.left_child_node().unwrap().frequency(), 5);
    assert_eq!(merged_node.right_child_leaf(), Some(&leaf_c));
}
