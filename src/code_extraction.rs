use crate::tree_construction::HuffmanNode;
use std::collections::HashMap;

pub type HuffmanCodeMap = HashMap<u8, String>;

fn extract_codes_recursive(node: &HuffmanNode, current_code: String) -> Vec<(u8, String)> {
    match node.symbol() {
        Some(symbol) => vec![(symbol, current_code)],
        None => {
            let mut codes = Vec::new();

            if let Some(left) = node.left_child() {
                codes.extend(extract_codes_recursive(left, format!("{current_code}0")));
            }
            if let Some(right) = node.right_child() {
                codes.extend(extract_codes_recursive(right, format!("{current_code}1")));
            }

            codes
        }
    }
}

pub fn extract_huffman_codes(tree: &HuffmanNode) -> HuffmanCodeMap {
    if tree.is_leaf() {
        tree.symbol()
            .map(|symbol| [(symbol, "0".to_string())].into_iter().collect())
            .unwrap_or_default()
    } else {
        extract_codes_recursive(tree, String::new())
            .into_iter()
            .collect()
    }
}
