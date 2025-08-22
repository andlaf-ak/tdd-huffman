use tdd_huffman::*;
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
