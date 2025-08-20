use tdd_huffman::{merge_leaf_nodes, merge_with_leaf_node};

#[test]
fn merge_two_leaf_nodes() {
    let left_node = (65u8, 3usize); // 'A' with frequency 3
    let right_node = (66u8, 2usize); // 'B' with frequency 2

    let merged_node = merge_leaf_nodes(left_node, right_node);

    // The merged node should have combined frequency (3 + 2 = 5)
    assert_eq!(merged_node.frequency(), 5);
    // Internal nodes should not have a symbol value
    assert!(merged_node.symbol().is_none());
    // Should have two child nodes
    assert!(merged_node.left_child_node().is_some());
    assert!(merged_node.right_child_node().is_some());
    // The child nodes should be leaves with the original data
    let left_child = merged_node.left_child_node().unwrap();
    let right_child = merged_node.right_child_node().unwrap();
    assert_eq!(left_child.as_leaf(), Some(left_node));
    assert_eq!(right_child.as_leaf(), Some(right_node));
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
    assert!(merged_node.left_child_node().is_some());
    assert!(merged_node.right_child_node().is_some());

    let left_child = merged_node.left_child_node().unwrap();
    let right_child = merged_node.right_child_node().unwrap();

    // Left child should be the internal node with frequency 5
    assert_eq!(left_child.frequency(), 5);
    assert!(left_child.symbol().is_none());

    // Right child should be the leaf with symbol C and frequency 4
    assert_eq!(right_child.as_leaf(), Some(leaf_c));
}
