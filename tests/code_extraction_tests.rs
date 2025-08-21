use tdd_huffman::{extract_huffman_codes, HuffmanNode, build_huffman_tree, count_byte_frequencies};

#[test]
fn single_node_generates_single_bit_code() {
    // This test covers the edge case where we have only one symbol
    // In this case, the tree is just a single leaf node
    // The convention for single-symbol Huffman codes is to use "0" or "1"
    // (since we need at least 1 bit per symbol)

    let single_node = HuffmanNode::new_leaf(65u8, 10usize); // 'A' appears 10 times

    let codes = extract_huffman_codes(&single_node);

    // Should have exactly one entry
    assert_eq!(codes.len(), 1);

    // Should map symbol 'A' (65) to a single-bit code
    assert!(codes.contains_key(&65u8));
    let code = codes.get(&65u8).unwrap();

    // Code should be exactly 1 bit (either "0" or "1")
    assert_eq!(code.len(), 1);
    assert!(
        code == "0" || code == "1",
        "Single symbol should have 1-bit code, got: {}",
        code
    );
}

#[test]
fn two_nodes_generate_complementary_codes() {
    // This test covers a simple binary tree with two symbols
    // The tree structure should be:
    //       root
    //      /    \
    //   A(65)   B(66)
    // Expected codes: A="0", B="1" (or A="1", B="0")

    let left_leaf = HuffmanNode::new_leaf(65u8, 3usize); // 'A' appears 3 times
    let right_leaf = HuffmanNode::new_leaf(66u8, 5usize); // 'B' appears 5 times
    let tree_root = HuffmanNode::new_internal(left_leaf, right_leaf);

    let codes = extract_huffman_codes(&tree_root);

    // Should have exactly two entries
    assert_eq!(codes.len(), 2);

    // Both symbols should be present
    assert!(codes.contains_key(&65u8)); // 'A'
    assert!(codes.contains_key(&66u8)); // 'B'

    let code_a = codes.get(&65u8).unwrap();
    let code_b = codes.get(&66u8).unwrap();

    // Both codes should be exactly 1 bit
    assert_eq!(code_a.len(), 1);
    assert_eq!(code_b.len(), 1);

    // Codes should be complementary (one "0", one "1")
    assert!(
        (code_a == "0" && code_b == "1") || (code_a == "1" && code_b == "0"),
        "Expected complementary codes, got A='{}' and B='{}'",
        code_a,
        code_b
    );
}

#[test]
fn multiple_nodes_generate_prefix_free_codes() {
    // This test verifies that a tree with multiple symbols generates prefix-free codes
    // Tree structure:
    //         root
    //        /    \
    //    internal  C(67)
    //    /     \
    // A(65)   B(66)
    // Expected codes: A="00", B="01", C="1" (or similar prefix-free assignment)
    
    let leaf_a = HuffmanNode::new_leaf(65u8, 1usize); // 'A' appears 1 time
    let leaf_b = HuffmanNode::new_leaf(66u8, 1usize); // 'B' appears 1 time
    let internal_left = HuffmanNode::new_internal(leaf_a, leaf_b);
    let leaf_c = HuffmanNode::new_leaf(67u8, 2usize); // 'C' appears 2 times
    let tree_root = HuffmanNode::new_internal(internal_left, leaf_c);
    
    let codes = extract_huffman_codes(&tree_root);
    
    // Should have exactly three entries
    assert_eq!(codes.len(), 3);
    
    // All symbols should be present
    assert!(codes.contains_key(&65u8)); // 'A'
    assert!(codes.contains_key(&66u8)); // 'B'
    assert!(codes.contains_key(&67u8)); // 'C'
    
    let code_a = codes.get(&65u8).unwrap();
    let code_b = codes.get(&66u8).unwrap();
    let code_c = codes.get(&67u8).unwrap();
    
    // Verify prefix-free property: no code should be a prefix of another
    let all_codes = vec![code_a, code_b, code_c];
    for (i, code1) in all_codes.iter().enumerate() {
        for (j, code2) in all_codes.iter().enumerate() {
            if i != j {
                assert!(
                    !code1.starts_with(*code2) && !code2.starts_with(*code1),
                    "Codes are not prefix-free: '{}' and '{}' violate prefix-free property",
                    code1, code2
                );
            }
        }
    }
    
}

// Property-based tests to ensure prefix-free property holds for any tree construction
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Helper function to validate prefix-free property without being part of production code
    fn validates_prefix_free_property(codes: &std::collections::HashMap<u8, String>) -> bool {
        let all_codes: Vec<&String> = codes.values().collect();
        
        for (i, code1) in all_codes.iter().enumerate() {
            for (j, code2) in all_codes.iter().enumerate() {
                if i != j {
                    // No code should be a prefix of another
                    if code1.starts_with(*code2) || code2.starts_with(*code1) {
                        return false;
                    }
                }
            }
        }
        true
    }

    proptest! {
        #[test]
        fn generated_codes_are_always_prefix_free(
            symbols in prop::collection::vec(0u8..255, 2..6),
            frequencies in prop::collection::vec(1usize..100, 2..6),
        ) {
            prop_assume!(symbols.len() == frequencies.len());
            prop_assume!(symbols.len() >= 2);
            
            // Create unique symbols (no duplicates)
            let mut unique_symbols = symbols.clone();
            unique_symbols.sort();
            unique_symbols.dedup();
            prop_assume!(unique_symbols.len() >= 2);
            
            // Build input data by repeating each symbol according to its frequency
            let mut input_data = Vec::new();
            for (i, &symbol) in unique_symbols.iter().enumerate() {
                let freq = frequencies.get(i).unwrap_or(&1);
                for _ in 0..*freq {
                    input_data.push(symbol);
                }
            }
            
            // Use our established count_byte_frequencies function
            let frequency_map = count_byte_frequencies(&input_data);
            
            // Use our tested build_huffman_tree function
            let tree = build_huffman_tree(&frequency_map);
            let codes = extract_huffman_codes(&tree);
            
            // Validate prefix-free property
            prop_assert!(validates_prefix_free_property(&codes));
            
            // All codes should be non-empty
            for code in codes.values() {
                prop_assert!(!code.is_empty());
            }
            
            // Should have one code per unique symbol
            prop_assert_eq!(codes.len(), unique_symbols.len());
        }
    }
}
