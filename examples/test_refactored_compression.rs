use std::io::Cursor;
use tdd_huffman::compress;

fn main() {
    let test_string = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.";
    let mut compressed_data = Vec::new();

    compress(Cursor::new(test_string.as_bytes()), &mut compressed_data)
        .expect("Compression should succeed");

    println!("Original: '{}'", test_string);
    println!("Compressed: {} bytes", compressed_data.len());
    println!("Original size: {} bytes", test_string.len());
    let compression_ratio = compressed_data.len() as f64 / test_string.len() as f64;
    println!("Compression ratio: {:.3}", compression_ratio);
}
