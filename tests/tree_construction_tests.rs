use tdd_huffman::{
    build_huffman_tree, merge_leaf_nodes, merge_nodes, ByteFrequencyMap, HuffmanNode,
};

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
    let leaf_c_node = HuffmanNode::new_leaf(leaf_c.0, leaf_c.1);
    let merged_node = merge_nodes(internal_node, leaf_c_node);

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
fn single_byte_creates_tree_with_one_leaf_node() {
    let mut frequency_map = ByteFrequencyMap::new();
    frequency_map.insert(65u8, 5usize); // 'A' appears 5 times

    let tree = build_huffman_tree(&frequency_map);

    // For a single byte, the tree should be just one leaf node
    assert_eq!(tree.frequency(), 5);
    assert_eq!(tree.symbol(), Some(65u8));
    assert!(tree.left_child().is_none());
    assert!(tree.right_child().is_none());
    assert_eq!(tree.as_leaf(), Some((65u8, 5usize)));
}

#[test]
fn two_bytes_create_tree_with_one_internal_node_and_two_leaves() {
    let mut frequency_map = ByteFrequencyMap::new();
    frequency_map.insert(65u8, 3usize); // 'A' appears 3 times
    frequency_map.insert(66u8, 7usize); // 'B' appears 7 times

    let tree = build_huffman_tree(&frequency_map);

    // Root should have total frequency (3 + 7 = 10)
    assert_eq!(tree.frequency(), 10);

    // Root should be internal node (no symbol)
    assert!(tree.symbol().is_none());
    assert!(tree.left_child().is_some());
    assert!(tree.right_child().is_some());

    // Tree should have exactly two leaf nodes with the original symbols
    let left_child = tree.left_child().unwrap();
    let right_child = tree.right_child().unwrap();

    // Both children should be leaves
    assert!(left_child.symbol().is_some());
    assert!(right_child.symbol().is_some());

    // Collect the two leaf symbols (order doesn't matter for this test)
    let mut leaf_data = vec![
        left_child.as_leaf().unwrap(),
        right_child.as_leaf().unwrap(),
    ];
    leaf_data.sort();

    // Should contain both original symbol/frequency pairs
    assert_eq!(leaf_data, vec![(65u8, 3usize), (66u8, 7usize)]);
}

#[test]
fn multiple_bytes_create_proper_binary_tree_structure() {
    let mut frequency_map = ByteFrequencyMap::new();
    frequency_map.insert(65u8, 5usize);  // 'A' appears 5 times
    frequency_map.insert(66u8, 2usize);  // 'B' appears 2 times
    frequency_map.insert(67u8, 1usize);  // 'C' appears 1 time
    frequency_map.insert(68u8, 3usize);  // 'D' appears 3 times

    let tree = build_huffman_tree(&frequency_map);

    // Root should have total frequency (5 + 2 + 1 + 3 = 11)
    assert_eq!(tree.frequency(), 11);

    // Root should be internal node (no symbol)
    assert!(tree.symbol().is_none());
    assert!(tree.left_child().is_some());
    assert!(tree.right_child().is_some());

    // Tree should have proper binary structure
    // - All internal nodes should have exactly 2 children
    // - All leaves should have no children
    // - All original symbols should be preserved as leaves
    assert!(validate_binary_tree_structure(&tree));

    // All original symbols should be findable as leaves in the tree
    let leaf_symbols = collect_leaf_symbols(&tree);
    let mut expected_symbols = vec![65u8, 66u8, 67u8, 68u8];
    expected_symbols.sort();
    let mut actual_symbols = leaf_symbols.clone();
    actual_symbols.sort();
    assert_eq!(actual_symbols, expected_symbols);

    // Total frequency should be preserved
    let total_leaf_frequency: usize = collect_leaf_frequencies(&tree).iter().sum();
    assert_eq!(total_leaf_frequency, 11);
}

// Helper function to validate proper binary tree structure
fn validate_binary_tree_structure(node: &HuffmanNode) -> bool {
    match (node.symbol(), node.left_child(), node.right_child()) {
        // Leaf nodes: have symbol, no children
        (Some(_), None, None) => true,
        // Internal nodes: no symbol, exactly 2 children
        (None, Some(left), Some(right)) => {
            validate_binary_tree_structure(left) && validate_binary_tree_structure(right)
        }
        // Invalid: any other combination
        _ => false,
    }
}

// Helper function to collect all leaf symbols
fn collect_leaf_symbols(node: &HuffmanNode) -> Vec<u8> {
    let mut symbols = Vec::new();
    collect_leaf_symbols_recursive(node, &mut symbols);
    symbols
}

fn collect_leaf_symbols_recursive(node: &HuffmanNode, symbols: &mut Vec<u8>) {
    if let Some(symbol) = node.symbol() {
        symbols.push(symbol);
    } else {
        if let Some(left) = node.left_child() {
            collect_leaf_symbols_recursive(left, symbols);
        }
        if let Some(right) = node.right_child() {
            collect_leaf_symbols_recursive(right, symbols);
        }
    }
}

// Helper function to collect all leaf frequencies
fn collect_leaf_frequencies(node: &HuffmanNode) -> Vec<usize> {
    let mut frequencies = Vec::new();
    collect_leaf_frequencies_recursive(node, &mut frequencies);
    frequencies
}

fn collect_leaf_frequencies_recursive(node: &HuffmanNode, frequencies: &mut Vec<usize>) {
    if node.symbol().is_some() {
        frequencies.push(node.frequency());
    } else {
        if let Some(left) = node.left_child() {
            collect_leaf_frequencies_recursive(left, frequencies);
        }
        if let Some(right) = node.right_child() {
            collect_leaf_frequencies_recursive(right, frequencies);
        }
    }
}

// Property-based test to ensure frequency invariant holds for any tree construction
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Helper function to validate frequency invariant without being part of production code
    fn validates_frequency_invariant(node: &HuffmanNode) -> bool {
        // Base case: leaf nodes always satisfy the invariant
        if node.symbol().is_some() {
            return true;
        }

        // For internal nodes, check that frequency equals sum of children
        match (node.left_child(), node.right_child()) {
            (Some(left), Some(right)) => {
                let children_sum = left.frequency() + right.frequency();
                node.frequency() == children_sum
                    && validates_frequency_invariant(left)
                    && validates_frequency_invariant(right)
            }
            _ => false, // Internal nodes must have both children
        }
    }

    proptest! {
        #[test]
        fn parent_node_frequency_equals_sum_of_children_frequencies(
            symbol1 in 0u8..255,
            symbol2 in 0u8..255,
            freq1 in 1usize..1000,
            freq2 in 1usize..1000,
        ) {
            // Test that any merge of leaf nodes satisfies the frequency invariant
            let leaf_node1 = (symbol1, freq1);
            let leaf_node2 = (symbol2, freq2);
            let merged_tree = merge_leaf_nodes(leaf_node1, leaf_node2);

            prop_assert!(validates_frequency_invariant(&merged_tree));
        }

        #[test]
        fn complex_tree_maintains_frequency_invariant(
            symbols in prop::collection::vec(0u8..255, 3..6),
            frequencies in prop::collection::vec(1usize..100, 3..6),
        ) {
            prop_assume!(symbols.len() == frequencies.len());
            prop_assume!(symbols.len() >= 3);

            // Create leaf nodes
            let mut nodes = Vec::new();
            for (symbol, freq) in symbols.iter().zip(frequencies.iter()) {
                nodes.push(HuffmanNode::new_leaf(*symbol, *freq));
            }

            // Build a tree by progressively merging nodes
            let mut current = nodes.pop().unwrap();
            while let Some(next) = nodes.pop() {
                current = merge_nodes(current, next);
            }

            prop_assert!(validates_frequency_invariant(&current));
        }
    }
}
