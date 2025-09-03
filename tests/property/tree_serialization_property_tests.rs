use proptest::prelude::*;
use std::io::Cursor;
use tdd_huffman::*;

// Helper function to compare tree structure and symbols (ignoring frequencies)
fn trees_structurally_equal(tree1: &HuffmanNode, tree2: &HuffmanNode) -> bool {
    // Compare symbols (for leaf nodes)
    if tree1.symbol() != tree2.symbol() {
        return false;
    }

    // If both are leaf nodes, they're equal (symbols already compared)
    if tree1.symbol().is_some() && tree2.symbol().is_some() {
        return true;
    }

    // If one is leaf and one is internal, they're not equal
    if tree1.symbol().is_some() != tree2.symbol().is_some() {
        return false;
    }

    // Both are internal nodes - compare children recursively
    match (tree1.left_child(), tree2.left_child()) {
        (Some(left1), Some(left2)) => {
            if !trees_structurally_equal(left1, left2) {
                return false;
            }
        }
        (None, None) => {}
        _ => return false,
    }

    match (tree1.right_child(), tree2.right_child()) {
        (Some(right1), Some(right2)) => trees_structurally_equal(right1, right2),
        (None, None) => true,
        _ => false,
    }
}

proptest! {
    #[test]
    fn serialize_then_deserialize_equals_original_tree(_dummy in 0u8..1) {
        // Use the "she sells seashells on the seashore" example
        let input = b"she sells seashells on the seashore";
        let freq_map = count_byte_frequencies(input);
        let tree = build_huffman_tree(&freq_map);

        // Serialize the tree to bits using a Vec<u8> buffer
        let mut serialized_bits = Vec::new();
        {
            let mut bit_stream = OutputBitStream::new(&mut serialized_bits);
            serialize_tree_to_bits(&tree, &mut bit_stream)
                .expect("Serialization should succeed");
            bit_stream.flush().expect("Flush should succeed");
        }

        // Create an InputBitStream from the serialized data
        let cursor = Cursor::new(serialized_bits);
        let mut bit_stream = InputBitStream::new(cursor);

        // Deserialize back to a tree
        let deserialized_tree = deserialize_tree(&mut bit_stream)
            .expect("Deserialization should succeed");

        // Check that the deserialized tree has the same structure and symbols
        prop_assert!(trees_structurally_equal(&tree, &deserialized_tree));
    }
}
