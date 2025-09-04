use crate::output_bit_stream::OutputBitStream;
use crate::tree_construction::HuffmanNode;
use std::io::Write;

pub fn serialize_tree_to_bits<W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    match tree.is_leaf() {
        true => serialize_leaf_to_bits(tree, bit_stream),
        false => serialize_internal_to_bits(tree, bit_stream),
    }
}

fn serialize_leaf_to_bits<W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    let symbol = tree.symbol().expect("Leaf node must have a symbol");

    std::iter::once(1)
        .chain((0..8).rev().map(|i| (symbol >> i) & 1))
        .try_for_each(|bit| bit_stream.write_bit(bit))
}

fn serialize_internal_to_bits<W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    bit_stream
        .write_bit(0)
        .and_then(|_| {
            tree.left_child()
                .map(|left| serialize_tree_to_bits(left, bit_stream))
                .unwrap_or(Ok(()))
        })
        .and_then(|_| {
            tree.right_child()
                .map(|right| serialize_tree_to_bits(right, bit_stream))
                .unwrap_or(Ok(()))
        })
}

pub fn serialize_tree(tree: &HuffmanNode) -> String {
    let mut output = Vec::new();
    let mut bit_stream = OutputBitStream::new(&mut output);

    serialize_tree_to_bits(tree, &mut bit_stream).expect("Failed to serialize tree to bits");

    let bits_written = count_tree_bits(tree);

    bit_stream.flush().expect("Failed to flush bit stream");

    bits_to_string(&output, bits_written)
}

fn count_tree_bits(tree: &HuffmanNode) -> usize {
    if tree.is_leaf() {
        9
    } else {
        1 + tree.left_child().map_or(0, count_tree_bits)
            + tree.right_child().map_or(0, count_tree_bits)
    }
}

fn bits_to_string(bytes: &[u8], num_bits: usize) -> String {
    bytes
        .iter()
        .flat_map(|&byte| (0..8).rev().map(move |i| (byte >> i) & 1))
        .take(num_bits)
        .map(|bit| if bit == 1 { '1' } else { '0' })
        .collect()
}
