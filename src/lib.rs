//! TDD Huffman Compression Library
//!
//! This library implements Huffman compression using Test-Driven Development.

pub mod frequency_map;

// Re-export main functions for easier access
pub use frequency_map::count_byte_frequencies;
