use crate::tree_construction::HuffmanNode;
use std::collections::HashMap;

/// Type alias for mapping symbols to their Huffman codes
pub type HuffmanCodeMap = HashMap<u8, String>;

/// Extract Huffman codes from a Huffman tree
///
/// For single-symbol trees, assigns "0". Multi-symbol trees are not yet supported.
pub fn extract_huffman_codes(tree: &HuffmanNode) -> HuffmanCodeMap {
    let mut symbol_to_code = HuffmanCodeMap::new();

    if tree.is_leaf() {
        // Single symbol case: assign 1-bit code
        if let Some(symbol) = tree.symbol() {
            symbol_to_code.insert(symbol, "0".to_string());
        }
    } else {
        // Multi-symbol case: TODO - implement tree traversal
        // This will require recursive traversal with path tracking
    }

    symbol_to_code
}
