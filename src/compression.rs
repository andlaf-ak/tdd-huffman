//! Complete Huffman compression functionality
//!
//! This module provides end-to-end compression by combining all the individual
//! Huffman compression components into a single pipeline.

use crate::{
    code_extraction::extract_huffman_codes,
    frequency_map::count_byte_frequencies,
    output_bit_stream::OutputBitStream,
    tree_construction::build_huffman_tree,
    tree_serialization::serialize_tree,
};
use std::collections::HashMap;

/// Result of the compression operation containing compressed data and metadata
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

/// Compresses a string using Huffman coding with detailed metadata.
///
/// This function performs the complete Huffman compression pipeline:
/// 1. Builds a frequency map of the input
/// 2. Constructs a Huffman tree from the frequencies
/// 3. Extracts Huffman codes from the tree
/// 4. Serializes the tree structure
/// 5. Encodes the input using the Huffman codes
///
/// # Arguments
///
/// * `input` - The string to compress
///
/// # Returns
///
/// A `CompressionResult` containing the compressed data and metadata
pub fn compress_string_with_details(input: &str) -> CompressionResult {
    let input_bytes = input.as_bytes();
    let original_bits = input_bytes.len() * 8;
    
    // Step 1: Build frequency map
    let frequency_map = count_byte_frequencies(input_bytes);
    
    // Step 2: Build Huffman tree
    let tree = build_huffman_tree(&frequency_map);
    
    // Step 3: Extract Huffman codes
    let codes = extract_huffman_codes(&tree);
    
    // Step 4: Serialize the tree
    let serialized_tree = serialize_tree(&tree);
    
    // Step 5: Encode the input using Huffman codes
    let mut output = Vec::new();
    let mut bit_stream = OutputBitStream::new(&mut output);
    
    // Write the serialized tree bits
    for bit_char in serialized_tree.chars() {
        let bit = if bit_char == '1' { 1 } else { 0 };
        bit_stream.write_bit(bit).expect("Failed to write tree bit");
    }
    
    // Write the encoded data
    for &byte in input_bytes {
        if let Some(code) = codes.get(&byte) {
            for bit_char in code.chars() {
                let bit = if bit_char == '1' { 1 } else { 0 };
                bit_stream.write_bit(bit).expect("Failed to write data bit");
            }
        }
    }
    
    // Flush any remaining bits
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

/// Simple compression function that returns just the compressed bytes
pub fn compress_string(input: &str) -> Vec<u8> {
    compress_string_with_details(input).compressed_data
}
