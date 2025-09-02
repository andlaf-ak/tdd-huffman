use tdd_huffman::compress_string_with_details;

fn main() {
    let test_string = "hello world";
    let result = compress_string_with_details(test_string);
    
    println!("Original: '{}'", test_string);
    println!("Compressed: {} bytes", result.compressed_data.len());
    println!("Compression ratio: {:.3}", result.compression_ratio);
    println!("Huffman codes: {:?}", result.huffman_codes);
}
