//! TDD Huffman Compression Library
//!
//! This library implements Huffman compression using Test-Driven Development.

pub mod frequency_map;
pub mod node_selection;

// Re-export main functions and types for easier access
pub use frequency_map::{count_byte_frequencies, ByteFrequencyMap};
pub use node_selection::select_nodes;
