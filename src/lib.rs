//! TDD Huffman Compression Library
//!
//! This library implements Huffman compression using Test-Driven Development.

pub mod code_extraction;
pub mod frequency_map;
pub mod node_selection;
pub mod tree_construction;

// Re-export main functions and types for easier access
pub use code_extraction::{extract_huffman_codes, HuffmanCodeMap};
pub use frequency_map::{count_byte_frequencies, ByteFrequencyMap};
pub use node_selection::{select_nodes, NodeCollection, SymbolFrequency};
pub use tree_construction::{build_huffman_tree, merge_leaf_nodes, merge_nodes, HuffmanNode};
