use crate::tree_construction::HuffmanNode;

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
    let left = tree.left_child().map(serialize_tree).unwrap_or_default();
    let right = tree.right_child().map(serialize_tree).unwrap_or_default();

    format!("0{left}{right}")
}
