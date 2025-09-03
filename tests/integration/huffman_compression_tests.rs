use rstest::rstest;
use std::io::Cursor;
use tdd_huffman::{compress, decompress};

#[path = "test_utils.rs"]
mod test_utils;
use test_utils::assert_original_length_in_header;

#[rstest]
#[case::abracadabra("abracadabra - classic test case", "abracadabra")]
#[case::quick_brown_fox(
    "the quick brown fox - short sentence",
    "the quick brown fox jumped over the lazy dog"
)]
#[case::sixth_sick_sheik("sixth sick sheik - long repetitive text", "The sixth sick sheik's sixth sheep's sick. But if the sixth sick sheik's sixth sheep's sick, then surely the seventh sick sheik's seventh sheep's sicker still. So the sixth sick sheik's sixth sheep's sickness is less serious than the seventh sick sheik's seventh sheep's sickness, unless the sixth sick sheik's sixth sheep's sickness makes the sixth sick sheik's sixth sheep sicker than the seventh sick sheik's seventh sheep, in which case the sixth sick sheik should seek a skilled sheep surgeon to skillfully cure his sixth sheep's sickness swiftly.")]
fn test_compression_round_trip(#[case] test_name: &str, #[case] input: &str) {
    println!("\n🔬 Testing: {}", test_name);
    println!("Input: \"{}\"", input);
    println!("Input length: {} characters", input.len());

    let mut compressed_data = Vec::new();
    compress(Cursor::new(input.as_bytes()), &mut compressed_data)
        .expect("Compression should succeed");

    assert_original_length_in_header(&compressed_data, input.len());

    let mut decompressed_data = Vec::new();
    decompress(Cursor::new(&compressed_data), &mut decompressed_data)
        .expect("Decompression should succeed");

    let decompressed_string =
        String::from_utf8(decompressed_data).expect("Decompressed data should be valid UTF-8");

    assert_eq!(
        input, decompressed_string,
        "Round-trip should preserve original data"
    );

    if input.len() > 20 {
        println!(
            "Original: {} bytes, Compressed: {} bytes",
            input.len(),
            compressed_data.len()
        );
    }

    println!("✅ Round-trip test passed");
}

#[rstest]
#[case("a")]
#[case("aa")]
#[case("aaa")]
#[case("aaaa")]
#[case("aaaaa")]
fn test_single_character_repeated(#[case] input: &str) {
    println!("\n🔬 Testing single character repeated: \"{}\"", input);
    println!("Input length: {} characters", input.len());

    let mut compressed_data = Vec::new();
    compress(Cursor::new(input.as_bytes()), &mut compressed_data)
        .expect("Compression should succeed");

    assert_original_length_in_header(&compressed_data, input.len());

    let mut decompressed_data = Vec::new();
    decompress(Cursor::new(&compressed_data), &mut decompressed_data)
        .expect("Decompression should succeed");

    let decompressed_string =
        String::from_utf8(decompressed_data).expect("Decompressed data should be valid UTF-8");

    assert_eq!(
        input, decompressed_string,
        "Round-trip should preserve original data"
    );

    println!("✅ Single character test passed");
}

#[rstest]
#[case("ab")]
#[case("abc")]
#[case("abcd")]
#[case("abcde")]
#[case("abcdef")]
fn test_unique_characters(#[case] input: &str) {
    println!("\n🔬 Testing all unique characters: \"{}\"", input);
    println!("Input length: {} characters", input.len());

    let mut compressed_data = Vec::new();
    compress(Cursor::new(input.as_bytes()), &mut compressed_data)
        .expect("Compression should succeed");

    assert_original_length_in_header(&compressed_data, input.len());

    let mut decompressed_data = Vec::new();
    decompress(Cursor::new(&compressed_data), &mut decompressed_data)
        .expect("Decompression should succeed");

    let decompressed_string =
        String::from_utf8(decompressed_data).expect("Decompressed data should be valid UTF-8");

    assert_eq!(
        input, decompressed_string,
        "Round-trip should preserve original data"
    );

    println!(
        "Original: {} bytes, Compressed: {} bytes",
        input.len(),
        compressed_data.len()
    );
    println!("✅ Unique characters test passed");
}
