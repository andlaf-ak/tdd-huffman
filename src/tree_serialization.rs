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
            .map(|symbol| format!("1{:08b}", symbol))
            .expect("Leaf node must have a symbol")
    } else {
        unimplemented!("Internal node serialization not yet implemented")
    }
}
