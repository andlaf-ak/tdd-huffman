use tdd_huffman::merge_leaf_nodes;

#[test]
fn merge_two_leaf_nodes() {
    let left_node = (65u8, 3usize);  // 'A' with frequency 3
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
