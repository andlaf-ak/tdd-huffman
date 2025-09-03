use std::io::Cursor;
use tdd_huffman::*;

fn main() {
    // Test the new decompress function with a simple input
    let input = "hello world";
    println!("Original input: {}", input);

    // Compress the input
    let compressed_data = compress_string(input);
    println!("Compressed data length: {} bytes", compressed_data.len());

    // Decompress using the new decompress function
    let cursor = Cursor::new(compressed_data);
    let mut output = Vec::new();

    match decompress(cursor, &mut output) {
        Ok(()) => {
            let result = String::from_utf8(output).expect("Should be valid UTF-8");
            println!("Decompressed result: {}", result);
            println!("Match: {}", input == result);
        }
        Err(e) => {
            println!("Decompression failed: {}", e);
        }
    }
}
