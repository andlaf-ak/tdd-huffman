use tdd_huffman::{
    build_huffman_tree, count_byte_frequencies, extract_huffman_codes, serialize_tree,
    OutputBitStream,
};

#[test]
fn compress_abracadabra_achieves_better_than_73_percent_compression_ratio() {
    // Arrange: Input string "abracadabra"
    let input = "abracadabra";
    let input_bytes = input.as_bytes();
    let original_size_bits = input_bytes.len() * 8; // 11 bytes * 8 = 88 bits

    // Act: Perform complete Huffman compression
    let compressed_data = compress_string(input);

    // Assert: Compression ratio should be around 73% (compressed size / original size)
    let compression_ratio = (compressed_data.len() as f64) / (original_size_bits as f64);

    // Should achieve better than 73% compression (meaning compressed size < 73% of original)
    assert!(
        compression_ratio < 0.73,
        "Expected compression ratio < 0.73, but got {:.2}%",
        compression_ratio * 100.0
    );

    // Also verify we actually achieved compression
    assert!(
        compressed_data.len() < original_size_bits,
        "Compressed size ({} bits) should be smaller than original ({} bits)",
        compressed_data.len(),
        original_size_bits
    );
}

// Function that doesn't exist yet - this will cause compilation to fail (RED phase)
fn compress_string(input: &str) -> Vec<u8> {
    unimplemented!("Complete Huffman compression not yet implemented")
}
