use tdd_huffman::HuffmanCodeMap;

/// Helper function to calculate data encoding bits for a given input and Huffman codes
pub fn calculate_data_encoding_bits(huffman_codes: &HuffmanCodeMap, input: &str) -> usize {
    input
        .bytes()
        .map(|byte| huffman_codes.get(&byte).map_or(0, |code| code.len()))
        .sum()
}

/// Helper function to validate that compressed data contains the original length in the header
pub fn assert_original_length_in_header(compressed_data: &[u8], expected_length: usize) {
    assert!(
        compressed_data.len() >= 4,
        "Compressed data should be at least 4 bytes to include length header"
    );
    
    let length_bytes = &compressed_data[0..4];
    let stored_length = u32::from_le_bytes([
        length_bytes[0], 
        length_bytes[1], 
        length_bytes[2], 
        length_bytes[3]
    ]) as usize;
    
    assert_eq!(
        stored_length, 
        expected_length,
        "First 4 bytes should contain original data length: expected {}, got {}",
        expected_length,
        stored_length
    );
}
