use rstest::rstest;
use tdd_huffman::compress_string_with_details;

mod test_utils;
use test_utils::calculate_data_encoding_bits;

#[rstest]
#[case::abracadabra(
    "abracadabra - classic test case",
    "abracadabra", 
    75,    // expected_total_bits
    23,    // expected_data_encoding_bits  
    5      // tolerance
)]
#[case::quick_brown_fox(
    "the quick brown fox - short sentence", 
    "the quick brown fox jumped over the lazy dog",
    456,   // expected_total_bits (no compression due to many unique chars)
    194,   // expected_data_encoding_bits
    30     // tolerance
)]
#[case::sixth_sick_sheik(
    "sixth sick sheik - long repetitive text",
    "The sixth sick sheik's sixth sheep's sick. But if the sixth sick sheik's sixth sheep's sick, then surely the seventh sick sheik's seventh sheep's sicker still. So the sixth sick sheik's sixth sheep's sickness is less serious than the seventh sick sheik's seventh sheep's sickness, unless the sixth sick sheik's sixth sheep's sickness makes the sixth sick sheik's sixth sheep sicker than the seventh sick sheik's seventh sheep, in which case the sixth sick sheik should seek a skilled sheep surgeon to skillfully cure his sixth sheep's sickness swiftly.",
    2416,  // expected_total_bits (including tree overhead)
    2125,  // expected_data_encoding_bits
    50     // tolerance
)]
fn compress_string_achieves_target_compression(
    #[case] test_name: &str,
    #[case] input: &str,
    #[case] expected_total_bits: usize,
    #[case] expected_data_encoding_bits: usize,
    #[case] tolerance: usize,
) {
    println!("\n🔬 Testing: {}", test_name);
    println!("Input: \"{}\"", input);
    println!("Input length: {} characters", input.len());

    // Act: Perform compression
    let result = compress_string_with_details(input);

    // Verify target total compression
    let total_diff = if result.compressed_bits > expected_total_bits {
        result.compressed_bits - expected_total_bits
    } else {
        expected_total_bits - result.compressed_bits
    };

    assert!(
        total_diff <= tolerance,
        "Total compressed size should be around {} bits (±{}), but got {} bits (diff: {})",
        expected_total_bits,
        tolerance,
        result.compressed_bits,
        total_diff
    );

    // Verify target data encoding
    let data_encoding_bits = calculate_data_encoding_bits(&result.huffman_codes, input);
    let data_diff = if data_encoding_bits > expected_data_encoding_bits {
        data_encoding_bits - expected_data_encoding_bits
    } else {
        expected_data_encoding_bits - data_encoding_bits
    };

    assert!(
        data_diff <= tolerance,
        "Data encoding should be around {} bits (±{}), but got {} bits (diff: {})",
        expected_data_encoding_bits,
        tolerance,
        data_encoding_bits,
        data_diff
    );

    // Print results
    println!("✅ Results:");
    println!("   • Original: {} bits", result.original_bits);
    println!("   • Tree: {} bits", result.serialized_tree.len());
    println!("   • Data encoding: {} bits", data_encoding_bits);
    println!("   • Total compressed: {} bits", result.compressed_bits);
    println!(
        "   • Expected: {} ± {} bits",
        expected_total_bits, tolerance
    );
    println!("   • Ratio: {:.1}%", result.compression_ratio * 100.0);
}

#[rstest]
#[case("a")]
#[case("aa")]
#[case("aaa")]
#[case("aaaa")]
#[case("aaaaa")]
fn compress_single_character_repeated(#[case] input: &str) {
    println!("\n🔬 Testing single character repeated: \"{}\"", input);
    println!("Input length: {} characters", input.len());

    // Act: Perform compression
    let result = compress_string_with_details(input);

    // Should have exactly 1 unique character
    assert_eq!(result.frequency_map.len(), 1);
    assert_eq!(result.huffman_codes.len(), 1);

    // Single character should get a simple code
    let code = result.huffman_codes.values().next().unwrap();
    assert!(!code.is_empty(), "Code should not be empty");

    // Calculate data encoding bits
    let data_encoding_bits = calculate_data_encoding_bits(&result.huffman_codes, input);

    // Print results
    println!("✅ Results:");
    println!("   • Original: {} bits", result.original_bits);
    println!("   • Tree: {} bits", result.serialized_tree.len());
    println!("   • Data encoding: {} bits", data_encoding_bits);
    println!("   • Total compressed: {} bits", result.compressed_bits);
    println!("   • Character code: \"{}\"", code);
    println!("   • Ratio: {:.1}%", result.compression_ratio * 100.0);
}

#[rstest]
#[case("ab", 2)]
#[case("abc", 3)]
#[case("abcd", 4)]
#[case("abcde", 5)]
#[case("abcdef", 6)]
fn compress_all_unique_characters_various_lengths(
    #[case] input: &str,
    #[case] expected_unique: usize,
) {
    println!("\n🔬 Testing all unique characters: \"{}\"", input);
    println!("Input length: {} characters", input.len());
    println!("Expected unique characters: {}", expected_unique);

    // Act: Perform compression
    let result = compress_string_with_details(input);

    // Should have expected number of unique characters
    assert_eq!(result.frequency_map.len(), expected_unique);
    assert_eq!(result.huffman_codes.len(), expected_unique);

    // For all unique characters, compression should not be effective due to tree overhead
    assert!(
        result.compressed_bits >= result.original_bits,
        "With all unique characters, compression should not be effective. Expected >= {} bits, got {} bits",
        result.original_bits,
        result.compressed_bits
    );

    // Calculate data encoding bits
    let data_encoding_bits = calculate_data_encoding_bits(&result.huffman_codes, input);

    // Print results
    println!("✅ Results:");
    println!("   • Original: {} bits", result.original_bits);
    println!("   • Tree: {} bits", result.serialized_tree.len());
    println!("   • Data encoding: {} bits", data_encoding_bits);
    println!("   • Total compressed: {} bits", result.compressed_bits);
    println!("   • Unique characters: {}", result.frequency_map.len());
    println!("   • Ratio: {:.1}%", result.compression_ratio * 100.0);
    println!("   • Huffman codes:");
    for (char, code) in &result.huffman_codes {
        println!("     '{}' -> \"{}\"", char, code);
    }
}
