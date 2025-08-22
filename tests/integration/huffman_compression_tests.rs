use std::collections::HashMap;
use tdd_huffman::compress_string_with_details;

/// Helper function to calculate data encoding bits
fn calculate_data_encoding_bits(huffman_codes: &HashMap<u8, String>, input: &str) -> usize {
    input
        .bytes()
        .map(|byte| huffman_codes.get(&byte).map_or(0, |code| code.len()))
        .sum()
}

/// Test case structure for parameterized compression tests
#[derive(Debug)]
struct CompressionTestCase {
    name: &'static str,
    input: &'static str,
    expected_total_bits: usize,
    expected_data_encoding_bits: Option<usize>, // If provided, will check data encoding specifically
    tolerance: usize,
}

#[test]
fn compress_multiple_inputs_complete_pipeline_achieves_target_compression() {
    // Define test cases with different inputs and expected compression targets
    let test_cases = vec![
        CompressionTestCase {
            name: "abracadabra - classic test case",
            input: "abracadabra",
            expected_total_bits: 75, // Realistic total including tree overhead
            expected_data_encoding_bits: Some(23), // Target for just the data encoding
            tolerance: 5,
        },
        CompressionTestCase {
            name: "the quick brown fox - short sentence",
            input: "the quick brown fox jumped over the lazy dog",
            expected_total_bits: 456, // This case may not compress well due to many unique chars
            expected_data_encoding_bits: Some(194), // Data encoding target
            tolerance: 30,
        },
        CompressionTestCase {
            name: "sixth sick sheik - long repetitive text",
            input: "The sixth sick sheik's sixth sheep's sick. But if the sixth sick sheik's sixth sheep's sick, then surely the seventh sick sheik's seventh sheep's sicker still. So the sixth sick sheik's sixth sheep's sickness is less serious than the seventh sick sheik's seventh sheep's sickness, unless the sixth sick sheik's sixth sheep's sickness makes the sixth sick sheik's sixth sheep sicker than the seventh sick sheik's seventh sheep, in which case the sixth sick sheik should seek a skilled sheep surgeon to skillfully cure his sixth sheep's sickness swiftly.",
            expected_total_bits: 2416, // Total including tree overhead (~289) + data encoding (~2125)
            expected_data_encoding_bits: Some(2125), // Data encoding target  
            tolerance: 50,
        },
    ];

    println!("\n🔬 Testing Complete Huffman Compression Pipeline - Multiple Inputs");
    println!("===================================================================");

    for (test_index, test_case) in test_cases.iter().enumerate() {
        println!("\n📋 Test Case {}: {}", test_index + 1, test_case.name);
        println!("{}", "=".repeat(60));

        // Arrange
        let input = test_case.input;
        let expected_original_bits = input.len() * 8;

        // Act: Perform complete compression and get detailed results
        let result = compress_string_with_details(input);

        // Display verbose pipeline output for the test
        println!("=== Huffman Compression Pipeline ===");
        println!(
            "Input: \"{}{}\"",
            if input.len() > 50 {
                &input[..50]
            } else {
                input
            },
            if input.len() > 50 { "..." } else { "" }
        );
        println!("Input length: {} characters", input.len());
        println!(
            "Original size: {} bytes ({} bits)",
            input.len(),
            result.original_bits
        );
        println!();

        println!("Step 1: Frequency Analysis");
        println!("  Found {} unique characters", result.frequency_map.len());

        // Show top 10 most frequent characters
        let mut freq_sorted: Vec<_> = result.frequency_map.iter().collect();
        freq_sorted.sort_by(|a, b| b.1.cmp(a.1));

        println!("  Top frequent characters:");
        for (i, (&byte, &count)) in freq_sorted.iter().take(10).enumerate() {
            let char_display = if byte.is_ascii_graphic() || byte == b' ' {
                format!("'{}'", byte as char)
            } else {
                format!("\\x{:02x}", byte)
            };
            println!(
                "    {}. {} (byte {}): {} occurrences",
                i + 1,
                char_display,
                byte,
                count
            );
        }
        if result.frequency_map.len() > 10 {
            println!(
                "    ... and {} more characters",
                result.frequency_map.len() - 10
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
        println!("  Sample codes for top characters:");
        for (i, (&byte, _)) in freq_sorted.iter().take(5).enumerate() {
            if let Some(code) = result.huffman_codes.get(&byte) {
                let char_display = if byte.is_ascii_graphic() || byte == b' ' {
                    format!("'{}'", byte as char)
                } else {
                    format!("\\x{:02x}", byte)
                };
                println!("    {}. {} (byte {}): {}", i + 1, char_display, byte, code);
            }
        }
        if result.huffman_codes.len() > 5 {
            println!("    ... and {} more codes", result.huffman_codes.len() - 5);
        }
        println!();

        println!("Step 4: Tree Serialization");
        println!(
            "  Serialized tree length: {} bits",
            result.serialized_tree.len()
        );
        println!(
            "  Tree preview: {}{}",
            if result.serialized_tree.len() > 50 {
                &result.serialized_tree[..50]
            } else {
                &result.serialized_tree
            },
            if result.serialized_tree.len() > 50 {
                "..."
            } else {
                ""
            }
        );
        println!();

        let data_encoding_bits = calculate_data_encoding_bits(&result.huffman_codes, input);
        println!("Step 5: Data Encoding");
        println!("  Data encoding bits: {}", data_encoding_bits);

        // Show a sample of the encoded sequence for shorter inputs
        if input.len() <= 20 {
            let encoded_sequence: String = input
                .bytes()
                .filter_map(|byte| result.huffman_codes.get(&byte).map(|s| s.as_str()))
                .collect();
            println!("  Encoded sequence: {}", encoded_sequence);
        }
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
            if result.original_bits > result.compressed_bits {
                result.original_bits - result.compressed_bits
            } else {
                0
            },
            if result.compression_ratio < 1.0 {
                (1.0 - result.compression_ratio) * 100.0
            } else {
                0.0
            }
        );
        println!();

        // Assert: Verify the compression pipeline worked correctly

        // 1. Verify original size calculation
        assert_eq!(
            result.original_bits, expected_original_bits,
            "Test case '{}': Original size should be {} bits",
            test_case.name, expected_original_bits
        );

        // 2. Verify frequency map is not empty
        assert!(
            !result.frequency_map.is_empty(),
            "Test case '{}': Frequency map should not be empty",
            test_case.name
        );

        // 3. Verify Huffman codes were generated for all characters
        let unique_chars: std::collections::HashSet<u8> = input.bytes().collect();
        for &byte in &unique_chars {
            assert!(
                result.huffman_codes.contains_key(&byte),
                "Test case '{}': Huffman codes should contain byte {}",
                test_case.name,
                byte
            );

            let code = &result.huffman_codes[&byte];
            assert!(
                !code.is_empty() && code.chars().all(|c| c == '0' || c == '1'),
                "Test case '{}': Huffman code for byte {} should be a non-empty binary string, got '{}'",
                test_case.name, byte, code
            );
        }

        // 4. Verify tree serialization is not empty
        assert!(
            !result.serialized_tree.is_empty(),
            "Test case '{}': Serialized tree should not be empty",
            test_case.name
        );
        assert!(
            result.serialized_tree.chars().all(|c| c == '0' || c == '1'),
            "Test case '{}': Serialized tree should contain only '0' and '1' characters",
            test_case.name
        );

        // 5. Verify compression pipeline completed successfully (may not always reduce size)
        // Note: For inputs with many unique characters, Huffman may not achieve compression

        // The algorithm should still work correctly even if it doesn't compress
        println!(
            "Compression effectiveness: {}",
            if result.compressed_bits < result.original_bits {
                "✅ Achieved compression"
            } else {
                "⚠️  No compression achieved (normal for diverse inputs)"
            }
        );

        // 6. Verify target compression is achieved within tolerance
        let compression_diff = if result.compressed_bits > test_case.expected_total_bits {
            result.compressed_bits - test_case.expected_total_bits
        } else {
            test_case.expected_total_bits - result.compressed_bits
        };

        assert!(
            compression_diff <= test_case.tolerance,
            "Test case '{}': Total compressed size should be around {} bits (±{}), but got {} bits (diff: {})",
            test_case.name, test_case.expected_total_bits, test_case.tolerance, result.compressed_bits, compression_diff
        );

        // 7. If data encoding target is specified, verify it as well
        if let Some(expected_data_bits) = test_case.expected_data_encoding_bits {
            let data_encoding_bits = calculate_data_encoding_bits(&result.huffman_codes, input);
            let data_diff = if data_encoding_bits > expected_data_bits {
                data_encoding_bits - expected_data_bits
            } else {
                expected_data_bits - data_encoding_bits
            };

            assert!(
                data_diff <= test_case.tolerance,
                "Test case '{}': Data encoding should be around {} bits (±{}), but got {} bits (diff: {})",
                test_case.name, expected_data_bits, test_case.tolerance, data_encoding_bits, data_diff
            );
        }

        // 8. Verify compression ratio calculation is correct
        let expected_ratio = result.compressed_bits as f64 / result.original_bits as f64;
        assert!(
            (result.compression_ratio - expected_ratio).abs() < 0.001,
            "Test case '{}': Compression ratio calculation should be correct: expected {:.3}, got {:.3}",
            test_case.name, expected_ratio, result.compression_ratio
        );

        // 9. Verify compressed data is not empty
        assert!(
            !result.compressed_data.is_empty(),
            "Test case '{}': Compressed data should not be empty",
            test_case.name
        );

        println!("✅ Test case '{}' passed!", test_case.name);
        println!("📊 Results Summary:");
        println!("   • Original: {} bits", result.original_bits);
        println!(
            "   • Tree serialization: {} bits",
            result.serialized_tree.len()
        );
        println!("   • Data encoding: {} bits", data_encoding_bits);
        println!("   • Total compressed: {} bits", result.compressed_bits);
        println!(
            "   • Expected: {} ± {} bits",
            test_case.expected_total_bits, test_case.tolerance
        );
        println!("   • Ratio: {:.1}%", result.compression_ratio * 100.0);

        let space_saved = if result.compressed_bits < result.original_bits {
            result.original_bits - result.compressed_bits
        } else {
            0 // No space saved if compression increased size
        };

        let savings_percentage = if result.compressed_bits < result.original_bits {
            (1.0 - result.compression_ratio) * 100.0
        } else {
            0.0 // No savings if compression increased size
        };

        println!(
            "   • Saved: {} bits ({:.1}%)",
            space_saved, savings_percentage
        );

        if test_index < test_cases.len() - 1 {
            println!("\n{}", "─".repeat(60));
        }
    }

    println!("\n🎉 All parameterized compression tests passed!");
    println!(
        "Tested {} different input scenarios successfully",
        test_cases.len()
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

    // The algorithm should still produce compressed data (even if larger than original)
    assert!(!result.compressed_data.is_empty());
    
    // For all unique characters, compression should not be effective due to tree overhead
    assert!(
        result.compressed_bits >= result.original_bits,
        "With all unique characters, compression should not be effective. Expected >= {} bits, got {} bits",
        result.original_bits,
        result.compressed_bits
    );
}
