use crate::tree_construction::HuffmanNode;
use std::collections::HashMap;

pub type HuffmanCodeMap = HashMap<u8, String>;

pub fn extract_huffman_codes(tree: &HuffmanNode) -> HuffmanCodeMap {
    let mut codes = HuffmanCodeMap::new();

    if tree.is_leaf() {
        if let Some(symbol) = tree.symbol() {
            codes.insert(symbol, "0".to_string());
        }
    } else {
        extract_codes_recursive(tree, String::new(), &mut codes);
    }

    codes
}

fn extract_codes_recursive(node: &HuffmanNode, current_code: String, codes: &mut HuffmanCodeMap) {
    if let Some(symbol) = node.symbol() {
        codes.insert(symbol, current_code);
    } else {
        if let Some(left) = node.left_child() {
            extract_codes_recursive(left, format!("{current_code}0"), codes);
        }
        if let Some(right) = node.right_child() {
            extract_codes_recursive(right, format!("{current_code}1"), codes);
        }
    }
}
