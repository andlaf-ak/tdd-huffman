use crate::tree_construction::HuffmanNode;
use std::collections::HashMap;

/// Type alias for mapping symbols to their Huffman codes
pub type HuffmanCodeMap = HashMap<u8, String>;

/// Extract Huffman codes from a Huffman tree
///
/// For single-symbol trees, assigns "0". For multi-symbol trees, performs tree traversal.
pub fn extract_huffman_codes(tree: &HuffmanNode) -> HuffmanCodeMap {
    let mut codes = HuffmanCodeMap::new();

    if tree.is_leaf() {
        // Special case: single symbol gets "0"
        if let Some(symbol) = tree.symbol() {
            codes.insert(symbol, "0".to_string());
        }
    } else {
        // General case: traverse tree starting with empty code
        extract_codes_recursive(tree, String::new(), &mut codes);
    }

    codes
}

/// Recursively traverse the tree and assign codes based on path
fn extract_codes_recursive(node: &HuffmanNode, current_code: String, codes: &mut HuffmanCodeMap) {
    if let Some(symbol) = node.symbol() {
        // Leaf node: record the code for this symbol
        codes.insert(symbol, current_code);
    } else {
        // Internal node: traverse children with extended codes
        if let Some(left) = node.left_child() {
            extract_codes_recursive(left, format!("{current_code}0"), codes);
        }
        if let Some(right) = node.right_child() {
            extract_codes_recursive(right, format!("{current_code}1"), codes);
        }
    }
}
