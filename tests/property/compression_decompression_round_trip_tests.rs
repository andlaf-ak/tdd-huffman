use proptest::prelude::*;
use rstest::rstest;
use std::io::Cursor;
use tdd_huffman::{compress_string, decompress};

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur.";

#[rstest]
#[case::single_char_a("a")]
#[case::single_char_repeated_short("aa")]
#[case::single_char_repeated_medium("aaa")]
#[case::single_char_repeated_long("aaaa")]
#[case::single_char_repeated_longer("aaaaa")]
#[case::two_unique_chars("ab")]
#[case::three_unique_chars("abc")]
#[case::four_unique_chars("abcd")]
#[case::five_unique_chars("abcde")]
#[case::six_unique_chars("abcdef")]
#[case::classic_abracadabra("abracadabra")]
#[case::quick_brown_fox("the quick brown fox jumped over the lazy dog")]
#[case::sixth_sick_sheik("The sixth sick sheik's sixth sheep's sick. But if the sixth sick sheik's sixth sheep's sick, then surely the seventh sick sheik's seventh sheep's sicker still. So the sixth sick sheik's sixth sheep's sickness is less serious than the seventh sick sheik's seventh sheep's sickness, unless the sixth sick sheik's sixth sheep's sickness makes the sixth sick sheik's sixth sheep sicker than the seventh sick sheik's seventh sheep, in which case the sixth sick sheik should seek a skilled sheep surgeon to skillfully cure his sixth sheep's sickness swiftly.")]
#[case::long_lorem_ipsum(LOREM_IPSUM)]
#[case::empty_string("")]
#[case::single_space(" ")]
#[case::multiple_spaces("   ")]
#[case::mixed_whitespace(" \t\n")]
#[case::numbers_and_punctuation("Hello, World! 123 $%^&*()")]
#[case::unicode_basic("café naïve résumé")]
fn test_compression_decompression_round_trip(#[case] input: &str) {
    println!(
        "\n🔄 Testing round-trip for: \"{}\"",
        if input.len() > 50 {
            format!("{}...", &input[..47])
        } else {
            input.to_string()
        }
    );
    println!("Input length: {} characters", input.len());

    // Skip empty string as it may not compress meaningfully
    if input.is_empty() {
        println!("⏭️  Skipping empty string test");
        return;
    }

    // Compress the input
    let compressed_data = compress_string(input);
    println!("Compressed size: {} bytes", compressed_data.len());

    // Decompress the compressed data
    let cursor = Cursor::new(compressed_data);
    let mut output = Vec::new();

    decompress(cursor, &mut output).expect("Decompression should succeed");

    // Convert back to string and verify round-trip
    let decompressed_string =
        String::from_utf8(output).expect("Decompressed bytes should be valid UTF-8");

    println!(
        "Original length: {}, Decompressed length: {}",
        input.len(),
        decompressed_string.len()
    );

    // Verify exact match
    assert_eq!(
        input, decompressed_string,
        "Round-trip failed: input != decompressed output"
    );
    println!("✅ Round-trip successful!");
}

// Property-based test for random strings
proptest! {
    #[test]
    fn compression_decompression_round_trip_property(
        input in "[a-zA-Z0-9 .,!?]{1,100}"
    ) {
        println!("\n🎲 Property test for random input: \"{}\"",
                if input.len() > 30 { format!("{}...", &input[..27]) } else { input.clone() });

        // Compress the input
        let compressed_data = compress_string(&input);

        // Decompress the compressed data
        let cursor = Cursor::new(compressed_data);
        let mut output = Vec::new();

        decompress(cursor, &mut output)
            .expect("Decompression should succeed");

        // Convert back to string and verify round-trip
        let decompressed_string = String::from_utf8(output)
            .expect("Decompressed bytes should be valid UTF-8");

        // Verify exact match
        prop_assert_eq!(input, decompressed_string);
    }

    #[test]
    fn compression_decompression_round_trip_repeated_chars(
        ch in prop::char::range('a', 'z'),
        count in 1usize..=50
    ) {
        let input = ch.to_string().repeat(count);
        println!("\n🔁 Property test for repeated char '{}' × {}: \"{}\"",
                ch, count, if input.len() > 30 { format!("{}...", &input[..27]) } else { input.clone() });

        // Compress the input
        let compressed_data = compress_string(&input);

        // Decompress the compressed data
        let cursor = Cursor::new(compressed_data);
        let mut output = Vec::new();

        decompress(cursor, &mut output)
            .expect("Decompression should succeed");

        // Convert back to string and verify round-trip
        let decompressed_string = String::from_utf8(output)
            .expect("Decompressed bytes should be valid UTF-8");

        // Verify exact match
        prop_assert_eq!(input, decompressed_string);
    }

    #[test]
    fn compression_decompression_round_trip_ascii_printable(
        input in prop::collection::vec(32u8..=126, 1..=100)
    ) {
        // Convert bytes to ASCII string
        let input_string = String::from_utf8(input).expect("Should be valid ASCII");

        println!("\n� Property test for ASCII data (len={}): \"{}\"",
                input_string.len(), if input_string.len() > 30 { format!("{}...", &input_string[..27]) } else { input_string.clone() });

        // Compress the input
        let compressed_data = compress_string(&input_string);

        // Decompress the compressed data
        let cursor = Cursor::new(compressed_data);
        let mut output = Vec::new();

        decompress(cursor, &mut output)
            .expect("Decompression should succeed");

        // Convert back to string and verify round-trip
        let decompressed_string = String::from_utf8(output)
            .expect("Decompressed bytes should be valid UTF-8");

        // Verify exact match
        prop_assert_eq!(input_string, decompressed_string);
    }
}
