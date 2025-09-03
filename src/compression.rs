use crate::{
    code_extraction::extract_huffman_codes,
    frequency_map::count_byte_frequencies,
    output_bit_stream::OutputBitStream,
    tree_construction::build_huffman_tree,
    tree_serialization::{serialize_tree, serialize_tree_to_bits},
};
use std::collections::HashMap;

fn encode_input_stream<W: std::io::Write>(
    input_bytes: &[u8],
    codes: &HashMap<u8, String>,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    for &byte in input_bytes {
        if let Some(code) = codes.get(&byte) {
            for bit_char in code.chars() {
                let bit = if bit_char == '1' { 1 } else { 0 };
                bit_stream.write_bit(bit)?;
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct CompressionResult {
    pub compressed_data: Vec<u8>,
    pub original_bits: usize,
    pub compressed_bits: usize,
    pub compression_ratio: f64,
    pub frequency_map: HashMap<u8, usize>,
    pub huffman_codes: HashMap<u8, String>,
    pub serialized_tree: String,
}

pub fn compress_string_with_details(input: &str) -> CompressionResult {
    let input_bytes = input.as_bytes();
    let original_bits = input_bytes.len() * 8;

    let frequency_map = count_byte_frequencies(input_bytes);
    let tree = build_huffman_tree(&frequency_map);
    let codes = extract_huffman_codes(&tree);
    let serialized_tree = serialize_tree(&tree);

    let mut output = Vec::new();
    
    // Write the original data length as a 4-byte little-endian header
    let original_length = input_bytes.len() as u32;
    output.extend_from_slice(&original_length.to_le_bytes());
    
    let mut bit_stream = OutputBitStream::new(&mut output);

    serialize_tree_to_bits(&tree, &mut bit_stream).expect("Failed to serialize tree to bit stream");

    encode_input_stream(input_bytes, &codes, &mut bit_stream)
        .expect("Failed to encode input stream");

    bit_stream.flush().expect("Failed to flush bit stream");

    let compressed_bits = output.len() * 8;
    let compression_ratio = compressed_bits as f64 / original_bits as f64;

    CompressionResult {
        compressed_data: output,
        original_bits,
        compressed_bits,
        compression_ratio,
        frequency_map,
        huffman_codes: codes,
        serialized_tree,
    }
}

pub fn compress_string(input: &str) -> Vec<u8> {
    compress_string_with_details(input).compressed_data
}
