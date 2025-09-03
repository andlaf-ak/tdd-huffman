use std::io::Cursor;
use tdd_huffman::compress;

fn main() {
    let test_string = "hello world";
    let mut compressed_data = Vec::new();

    compress(Cursor::new(test_string.as_bytes()), &mut compressed_data)
        .expect("Compression should succeed");

    println!("Original: '{}'", test_string);
    println!("Compressed: {} bytes", compressed_data.len());
    println!("Original size: {} bytes", test_string.len());
    let compression_ratio = compressed_data.len() as f64 / test_string.len() as f64;
    println!("Compression ratio: {:.3}", compression_ratio);
}
