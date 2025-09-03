use crate::input_bit_stream::InputBitStream;
use crate::tree_construction::HuffmanNode;

pub fn decompress<R: std::io::Read>(
    tree: &HuffmanNode,
    _bit_stream: &mut InputBitStream<R>,
    original_length: usize,
) -> Vec<u8> {
    if tree.is_leaf() {
        decompress_single_symbol_tree(tree, original_length)
    } else {
        todo!("Multi-node tree decompression not implemented yet")
    }
}

fn decompress_single_symbol_tree(tree: &HuffmanNode, output_length: usize) -> Vec<u8> {
    let symbol = tree.symbol().expect("Leaf node must have a symbol");
    vec![symbol; output_length]
}
