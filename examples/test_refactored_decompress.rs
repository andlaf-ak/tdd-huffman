use std::io::Cursor;
use tdd_huffman::*;

fn main() {
    // Test the new decompress function with a simple input
    let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.";
    println!("Original input: {}", input);

    // Compress the input
    let mut compressed_data = Vec::new();
    compress(Cursor::new(input.as_bytes()), &mut compressed_data)
        .expect("Compression should succeed");
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
