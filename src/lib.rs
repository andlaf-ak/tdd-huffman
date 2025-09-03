//! TDD Huffman Compression Library
//!
//! This library implements Huffman compression using Test-Driven Development.

pub mod code_extraction;
pub mod compression;
pub mod constants;
pub mod decompression;
pub mod frequency_map;
pub mod input_bit_stream;
pub mod node_selection;
pub mod output_bit_stream;
pub mod tree_construction;
pub mod tree_deserialization;
pub mod tree_serialization;

pub use code_extraction::{extract_huffman_codes, HuffmanCodeMap};
pub use compression::compress;
pub use decompression::{decode_compressed_data, decompress};
pub use frequency_map::{count_byte_frequencies, ByteFrequencyMap};
pub use input_bit_stream::InputBitStream;
pub use node_selection::select_nodes;
pub use output_bit_stream::OutputBitStream;
pub use tree_construction::{build_huffman_tree, merge_leaf_nodes, merge_nodes, HuffmanNode};
pub use tree_deserialization::deserialize_tree;
pub use tree_serialization::{serialize_tree, serialize_tree_to_bits};
