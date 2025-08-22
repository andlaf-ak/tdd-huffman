use tdd_huffman::HuffmanCodeMap;

/// Helper function to calculate data encoding bits for a given input and Huffman codes
pub fn calculate_data_encoding_bits(huffman_codes: &HuffmanCodeMap, input: &str) -> usize {
    input
        .bytes()
        .map(|byte| huffman_codes.get(&byte).map_or(0, |code| code.len()))
        .sum()
}
