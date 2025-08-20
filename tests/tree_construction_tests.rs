use tdd_huffman::{merge_internal_and_leaf_nodes, merge_leaf_nodes, merge_nodes};

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
    assert!(merged_node.left_child().is_some());
    assert!(merged_node.right_child().is_some());
    // The child nodes should be leaves with the original data
    let left_child = merged_node.left_child().unwrap();
    let right_child = merged_node.right_child().unwrap();
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
    let merged_node = merge_internal_and_leaf_nodes(internal_node, leaf_c);

    // The new merged node should have combined frequency (5 + 4 = 9)
    assert_eq!(merged_node.frequency(), 9);
    // Should be an internal node (no symbol)
    assert!(merged_node.symbol().is_none());
    // Should have the internal node as left child and leaf as right child
    assert!(merged_node.left_child().is_some());
    assert!(merged_node.right_child().is_some());

    let left_child = merged_node.left_child().unwrap();
    let right_child = merged_node.right_child().unwrap();

    // Left child should be the internal node with frequency 5
    assert_eq!(left_child.frequency(), 5);
    assert!(left_child.symbol().is_none());

    // Right child should be the leaf with symbol C and frequency 4
    assert_eq!(right_child.as_leaf(), Some(leaf_c));
}

#[test]
fn merge_two_non_leaf_nodes() {
    // Create first internal node from two leaf nodes
    let leaf_a = (65u8, 2usize); // 'A' with frequency 2
    let leaf_b = (66u8, 3usize); // 'B' with frequency 3
    let internal_node_1 = merge_leaf_nodes(leaf_a, leaf_b); // Internal node with frequency 5

    // Create second internal node from two different leaf nodes
    let leaf_c = (67u8, 1usize); // 'C' with frequency 1
    let leaf_d = (68u8, 4usize); // 'D' with frequency 4
    let internal_node_2 = merge_leaf_nodes(leaf_c, leaf_d); // Internal node with frequency 5

    // Now merge the two internal nodes
    let merged_node = merge_nodes(internal_node_1, internal_node_2);

    // The merged node should have combined frequency (5 + 5 = 10)
    assert_eq!(merged_node.frequency(), 10);
    // Should be an internal node (no symbol)
    assert!(merged_node.symbol().is_none());
    // Should have two internal nodes as children
    assert!(merged_node.left_child().is_some());
    assert!(merged_node.right_child().is_some());

    let left_child = merged_node.left_child().unwrap();
    let right_child = merged_node.right_child().unwrap();

    // Both children should be internal nodes with frequency 5
    assert_eq!(left_child.frequency(), 5);
    assert!(left_child.symbol().is_none());
    assert_eq!(right_child.frequency(), 5);
    assert!(right_child.symbol().is_none());

    // Left child should contain the A and B leaves
    assert!(left_child.left_child().is_some());
    assert!(left_child.right_child().is_some());

    // Right child should contain the C and D leaves
    assert!(right_child.left_child().is_some());
    assert!(right_child.right_child().is_some());
}

#[test]
fn parent_node_frequency_equals_sum_of_children_frequencies() {
    // Test with simple two-leaf merge
    let leaf_a = (65u8, 7usize); // 'A' with frequency 7
    let leaf_b = (66u8, 3usize); // 'B' with frequency 3
    let simple_tree = merge_leaf_nodes(leaf_a, leaf_b);

    assert!(simple_tree.validates_frequency_invariant());

    // Test with more complex tree (internal + leaf)
    let leaf_c = (67u8, 5usize); // 'C' with frequency 5
    let complex_tree = merge_internal_and_leaf_nodes(simple_tree, leaf_c);

    assert!(complex_tree.validates_frequency_invariant());

    // Test with two internal nodes
    let leaf_d = (68u8, 2usize); // 'D' with frequency 2
    let leaf_e = (69u8, 4usize); // 'E' with frequency 4
    let second_internal = merge_leaf_nodes(leaf_d, leaf_e);
    let final_tree = merge_nodes(complex_tree, second_internal);

    assert!(final_tree.validates_frequency_invariant());
}
