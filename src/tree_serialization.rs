use crate::tree_construction::HuffmanNode;

/// Serialize a Huffman tree to binary string representation
///
/// Uses pre-order traversal:
/// - Leaf nodes: "1" followed by 8-bit symbol representation
/// - Internal nodes: "0" followed by left subtree, then right subtree
pub fn serialize_tree(tree: &HuffmanNode) -> String {
    match tree.is_leaf() {
        true => serialize_leaf(tree),
        false => serialize_internal(tree),
    }
}

fn serialize_leaf(tree: &HuffmanNode) -> String {
    tree.symbol()
        .map(|symbol| format!("1{symbol:08b}"))
        .expect("Leaf node must have a symbol")
}

fn serialize_internal(tree: &HuffmanNode) -> String {
    let left_serialized = tree.left_child()
        .map(serialize_tree)
        .unwrap_or_default();
    
    let right_serialized = tree.right_child()
        .map(serialize_tree)
        .unwrap_or_default();
    
    format!("0{left_serialized}{right_serialized}")
}
