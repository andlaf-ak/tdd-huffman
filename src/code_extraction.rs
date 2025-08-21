use crate::tree_construction::HuffmanNode;
use std::collections::HashMap;

/// Type alias for mapping symbols to their Huffman codes
pub type HuffmanCodeMap = HashMap<u8, String>;

/// Extract Huffman codes from a Huffman tree
///
/// For single-symbol trees, assigns "0". For multi-symbol trees, performs tree traversal.
pub fn extract_huffman_codes(tree: &HuffmanNode) -> HuffmanCodeMap {
    let mut symbol_to_code = HuffmanCodeMap::new();

    if tree.is_leaf() {
        // Single symbol case: assign 1-bit code
        if let Some(symbol) = tree.symbol() {
            symbol_to_code.insert(symbol, "0".to_string());
        }
    } else {
        // Multi-symbol case: traverse tree and assign codes
        extract_codes_recursive(tree, String::new(), &mut symbol_to_code);
    }

    symbol_to_code
}

/// Recursively traverse the tree and assign codes based on path
/// Left child gets "0" appended, right child gets "1" appended
fn extract_codes_recursive(node: &HuffmanNode, code: String, codes: &mut HuffmanCodeMap) {
    if node.is_leaf() {
        // Found a leaf: assign the accumulated code to this symbol
        if let Some(symbol) = node.symbol() {
            codes.insert(symbol, code);
        }
    } else {
        // Internal node: traverse children with extended codes
        if let Some(left_child) = node.left_child() {
            extract_codes_recursive(left_child, format!("{code}0"), codes);
        }
        if let Some(right_child) = node.right_child() {
            extract_codes_recursive(right_child, format!("{code}1"), codes);
        }
    }
}
