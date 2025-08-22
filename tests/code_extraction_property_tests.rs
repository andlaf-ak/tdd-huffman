use tdd_huffman::*;
use proptest::prelude::*;
use std::collections::HashMap;

// Helper function to build input data from symbol-frequency pairs
fn build_input_data(symbol_freq_pairs: Vec<(u8, usize)>) -> Vec<u8> {
    let mut input_data = Vec::new();
    let mut unique_pairs: HashMap<u8, usize> = HashMap::new();

    // Deduplicate symbols by using last frequency for each symbol
    for (symbol, freq) in symbol_freq_pairs {
        unique_pairs.insert(symbol, freq);
    }

    // Build input data by repeating each symbol according to its frequency
    for (symbol, freq) in unique_pairs {
        for _ in 0..freq {
            input_data.push(symbol);
        }
    }
    input_data
}

// Helper function to validate prefix-free property
fn validates_prefix_free_property(codes: &HashMap<u8, String>) -> bool {
    let all_codes: Vec<&String> = codes.values().collect();
    for (i, code1) in all_codes.iter().enumerate() {
        for (j, code2) in all_codes.iter().enumerate() {
            if i != j && (code1.starts_with(*code2) || code2.starts_with(*code1)) {
                return false;
            }
        }
    }
    true
}

proptest! {
    #[test]
    fn generated_codes_are_always_prefix_free(
        symbol_freq_pairs in prop::collection::vec((0u8..255, 1usize..100), 2..6),
    ) {
        prop_assume!(symbol_freq_pairs.len() >= 2);

        let input_data = build_input_data(symbol_freq_pairs);
        prop_assume!(input_data.len() >= 2);

        let frequency_map = count_byte_frequencies(&input_data);
        prop_assume!(frequency_map.len() >= 2); // Ensure unique symbols

        let tree = build_huffman_tree(&frequency_map);
        let codes = extract_huffman_codes(&tree);

        // Validate all properties
        prop_assert!(validates_prefix_free_property(&codes));
        prop_assert_eq!(codes.len(), frequency_map.len());

        // All codes should be non-empty
        for code in codes.values() {
            prop_assert!(!code.is_empty());
        }
    }

    #[test]
    fn more_frequent_symbols_get_shorter_or_equal_length_codes(
        symbol_freq_pairs in prop::collection::vec((0u8..255, 1usize..50), 2..8),
    ) {
        prop_assume!(symbol_freq_pairs.len() >= 2);

        let input_data = build_input_data(symbol_freq_pairs);
        prop_assume!(input_data.len() >= 2);

        let frequency_map = count_byte_frequencies(&input_data);
        prop_assume!(frequency_map.len() >= 2); // Ensure unique symbols

        let tree = build_huffman_tree(&frequency_map);
        let codes = extract_huffman_codes(&tree);

        // Sort symbols by frequency (descending) for comparison
        let mut sorted_pairs: Vec<(u8, usize)> = frequency_map.into_iter().collect();
        sorted_pairs.sort_by(|a, b| b.1.cmp(&a.1));

        // Verify frequency-based code length property
        for i in 0..sorted_pairs.len() {
            for j in (i+1)..sorted_pairs.len() {
                let (symbol_more_freq, freq_more) = sorted_pairs[i];
                let (symbol_less_freq, freq_less) = sorted_pairs[j];

                // Only check code length relationship if frequencies are strictly different
                if freq_more > freq_less {
                    let code_more_freq = codes.get(&symbol_more_freq).unwrap();
                    let code_less_freq = codes.get(&symbol_less_freq).unwrap();

                    prop_assert!(
                        code_more_freq.len() <= code_less_freq.len(),
                        "Symbol {} (freq={}, code='{}') should have shorter or equal length than symbol {} (freq={}, code='{}')",
                        symbol_more_freq, freq_more, code_more_freq,
                        symbol_less_freq, freq_less, code_less_freq
                    );
                }
            }
        }
    }
}
