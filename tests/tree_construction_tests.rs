use tdd_huffman::{merge_leaf_nodes, merge_nodes, HuffmanNode};

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
