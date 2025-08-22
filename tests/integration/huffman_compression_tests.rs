use std::collections::HashMap;
use tdd_huffman::compress_string_with_details;

/// Helper function to calculate data encoding bits
fn calculate_data_encoding_bits(huffman_codes: &HashMap<u8, String>, input: &str) -> usize {
    input
        .bytes()
        .map(|byte| huffman_codes.get(&byte).map_or(0, |code| code.len()))
        .sum()
}

#[test]
fn compress_abracadabra_complete_pipeline_achieves_target_compression() {
    // RED Phase: Test the complete Huffman compression pipeline
    // Note: "abracadabra" has good frequency distribution (5 'a's, 2 'b's, 2 'r's, 1 'c', 1 'd')
    // Data encoding should be efficient (~23 bits), but tree overhead adds to total size

    // Arrange
    let input = "abracadabra";
    let expected_original_bits = 88; // 11 characters * 8 bits each
    let target_data_encoding_bits = 23; // Target for just the data encoding part
    let max_acceptable_total_bits = 75; // Realistic total including tree overhead

    println!("\n🔬 Testing Complete Huffman Compression Pipeline");
    println!("================================================");

    // Act: Perform complete compression and get detailed results
    let result = compress_string_with_details(input);

    // Display verbose pipeline output for the test
    println!("=== Huffman Compression Pipeline ===");
    println!("Input: \"{}\"", input);
    println!(
        "Original size: {} bytes ({} bits)",
        input.len(),
        result.original_bits
    );
    println!();

    println!("Step 1: Frequency Analysis");
    for (&byte, &count) in &result.frequency_map {
        println!(
            "  '{}' (byte {}): {} occurrences",
            byte as char, byte, count
        );
    }
    println!();

    println!("Step 2: Huffman Tree Construction");
    println!(
        "  Tree built successfully with {} unique symbols",
        result.frequency_map.len()
    );
    println!();

    println!("Step 3: Huffman Code Generation");
    for (&byte, code) in &result.huffman_codes {
        println!("  '{}' (byte {}): {}", byte as char, byte, code);
    }
    println!();

    println!("Step 4: Tree Serialization");
    println!("  Serialized tree: {}", result.serialized_tree);
    println!("  Tree size: {} bits", result.serialized_tree.len());
    println!();

    let data_encoding_bits = calculate_data_encoding_bits(&result.huffman_codes, input);
    println!("Step 5: Data Encoding");
    let encoded_sequence: String = input
        .bytes()
        .filter_map(|byte| result.huffman_codes.get(&byte).map(|s| s.as_str()))
        .collect();
    println!("  Encoded sequence: {}", encoded_sequence);
    println!("  Data encoding bits: {}", data_encoding_bits);
    println!();

    println!("=== Compression Results ===");
    println!("Original size: {} bits", result.original_bits);
    println!("Tree serialization: {} bits", result.serialized_tree.len());
    println!("Data encoding: {} bits", data_encoding_bits);
    println!("Total compressed: {} bits", result.compressed_bits);
    println!(
        "Compression ratio: {:.2}% ({} / {})",
        result.compression_ratio * 100.0,
        result.compressed_bits,
        result.original_bits
    );
    println!(
        "Space saved: {} bits ({:.1}%)",
        result.original_bits - result.compressed_bits,
        (1.0 - result.compression_ratio) * 100.0
    );
    println!();

    // Assert: Verify the compression pipeline worked correctly

    // 1. Verify original size calculation
    assert_eq!(
        result.original_bits, expected_original_bits,
        "Original size should be {} bits",
        expected_original_bits
    );

    // 2. Verify frequency map contains expected characters
    let expected_chars = ['a', 'b', 'r', 'c', 'd'];
    for &ch in &expected_chars {
        assert!(
            result.frequency_map.contains_key(&(ch as u8)),
            "Frequency map should contain character '{}'",
            ch
        );
    }

    // 3. Verify specific frequency counts for "abracadabra"
    assert_eq!(
        result.frequency_map[&(b'a')],
        5,
        "Character 'a' should appear 5 times"
    );
    assert_eq!(
        result.frequency_map[&(b'b')],
        2,
        "Character 'b' should appear 2 times"
    );
    assert_eq!(
        result.frequency_map[&(b'r')],
        2,
        "Character 'r' should appear 2 times"
    );
    assert_eq!(
        result.frequency_map[&(b'c')],
        1,
        "Character 'c' should appear 1 time"
    );
    assert_eq!(
        result.frequency_map[&(b'd')],
        1,
        "Character 'd' should appear 1 time"
    );

    // 4. Verify Huffman codes were generated for all characters
    for &ch in &expected_chars {
        assert!(
            result.huffman_codes.contains_key(&(ch as u8)),
            "Huffman codes should contain character '{}'",
            ch
        );

        let code = &result.huffman_codes[&(ch as u8)];
        assert!(
            !code.is_empty() && code.chars().all(|c| c == '0' || c == '1'),
            "Huffman code for '{}' should be a non-empty binary string, got '{}'",
            ch,
            code
        );
    }

    // 5. Verify more frequent characters get shorter codes
    let code_a = &result.huffman_codes[&(b'a')]; // appears 5 times (most frequent)
    let code_c = &result.huffman_codes[&(b'c')]; // appears 1 time (least frequent)

    assert!(
        code_a.len() <= code_c.len(),
        "Most frequent character 'a' should have code length <= least frequent 'c': '{}' vs '{}'",
        code_a,
        code_c
    );

    // 6. Verify tree serialization is not empty
    assert!(
        !result.serialized_tree.is_empty(),
        "Serialized tree should not be empty"
    );
    assert!(
        result.serialized_tree.chars().all(|c| c == '0' || c == '1'),
        "Serialized tree should contain only '0' and '1' characters"
    );

    // 7. Verify compression achieved
    assert!(
        result.compressed_bits < result.original_bits,
        "Compressed size ({} bits) should be smaller than original ({} bits)",
        result.compressed_bits,
        result.original_bits
    );

    // 8. Verify target compression is achieved
    // The total includes tree serialization + data encoding
    assert!(
        result.compressed_bits <= max_acceptable_total_bits,
        "Total compressed size should be <= {} bits, but got {} bits",
        max_acceptable_total_bits,
        result.compressed_bits
    );

    // Verify that the data encoding part specifically achieves the target
    let data_encoding_bits = calculate_data_encoding_bits(&result.huffman_codes, input);
    assert!(
        data_encoding_bits <= target_data_encoding_bits + 5, // Small tolerance
        "Data encoding should be around {} bits (±5), but got {} bits",
        target_data_encoding_bits,
        data_encoding_bits
    );

    // 9. Verify compression ratio is reasonable for this input size
    // Note: For short strings, tree overhead reduces compression effectiveness
    assert!(
        result.compression_ratio < 1.0, // Should be compressed, even if modestly
        "Compression ratio should be less than 100%, got {:.1}%",
        result.compression_ratio * 100.0
    );

    // Verify that we at least save some bits
    assert!(
        result.original_bits > result.compressed_bits,
        "Should save at least some bits: {} -> {}",
        result.original_bits,
        result.compressed_bits
    );

    // 10. Verify compressed data is not empty
    assert!(
        !result.compressed_data.is_empty(),
        "Compressed data should not be empty"
    );

    println!("\n✅ All compression pipeline assertions passed!");
    println!("📊 Final Results:");
    println!("   • Original: {} bits", result.original_bits);
    println!(
        "   • Tree serialization: {} bits",
        result.serialized_tree.len()
    );
    println!(
        "   • Data encoding: {} bits",
        calculate_data_encoding_bits(&result.huffman_codes, input)
    );
    println!("   • Total compressed: {} bits", result.compressed_bits);
    println!("   • Ratio: {:.1}%", result.compression_ratio * 100.0);
    println!(
        "   • Saved: {} bits ({:.1}%)",
        result.original_bits - result.compressed_bits,
        (1.0 - result.compression_ratio) * 100.0
    );
    println!(
        "   • Target data encoding achieved: {} ≤ {} ✓",
        calculate_data_encoding_bits(&result.huffman_codes, input),
        target_data_encoding_bits + 5
    );
}

#[test]
fn compress_single_character_string() {
    // Test edge case: single character repeated
    let input = "aaaa";
    let result = compress_string_with_details(input);

    // Should still achieve compression even with single character
    assert_eq!(result.frequency_map.len(), 1);
    assert!(result.huffman_codes.contains_key(&(b'a')));
    assert!(result.compressed_bits < result.original_bits);
}

#[test]
fn compress_all_unique_characters() {
    // Test case: all characters are unique (worst case for Huffman)
    let input = "abcde";
    let result = compress_string_with_details(input);

    // Should have 5 unique characters
    assert_eq!(result.frequency_map.len(), 5);

    // All characters should have codes
    for ch in input.chars() {
        assert!(result.huffman_codes.contains_key(&(ch as u8)));
    }

    // May not achieve great compression but should still work
    assert!(!result.compressed_data.is_empty());
}
