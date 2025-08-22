use crate::tree_construction::HuffmanNode;

/// Serialize a Huffman tree to binary string representation
///
/// Uses pre-order traversal:
/// - Leaf nodes: "1" followed by 8-bit symbol representation
/// - Internal nodes: "0" followed by left subtree, then right subtree
pub fn serialize_tree(tree: &HuffmanNode) -> String {
    if tree.is_leaf() {
        // For leaf nodes: "1" + 8-bit symbol representation
        tree.symbol()
            .map(|symbol| format!("1{symbol:08b}"))
            .expect("Leaf node must have a symbol")
    } else {
        // For internal nodes: "0" + left subtree + right subtree
        let mut result = String::from("0");
        
        if let Some(left_child) = tree.left_child() {
            result.push_str(&serialize_tree(left_child));
        }
        
        if let Some(right_child) = tree.right_child() {
            result.push_str(&serialize_tree(right_child));
        }
        
        result
    }
}
