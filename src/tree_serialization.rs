use crate::output_bit_stream::OutputBitStream;
use crate::tree_construction::HuffmanNode;
use std::io::Write;

// Converts a Huffman tree into a binary representation
// Leaf nodes: writes '1' bit followed by 8 bits representing the symbol
// Internal nodes: writes '0' bit followed by serialized left and right children
// This creates a compact binary format that can be stored with compressed data
pub fn serialize_tree_to_bits<W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    match tree.is_leaf() {
        true => serialize_leaf_to_bits(tree, bit_stream),
        false => serialize_internal_to_bits(tree, bit_stream),
    }
}

// Serializes a leaf node: writes marker bit '1' followed by the symbol's 8 bits
// Uses an iterator chain to write the marker bit plus all symbol bits in sequence
// The symbol bits are written from most significant to least significant
fn serialize_leaf_to_bits<W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    let symbol = tree.symbol().expect("Leaf node must have a symbol");

    std::iter::once(1)
        .chain((0..8).rev().map(|i| (symbol >> i) & 1))
        .try_for_each(|bit| bit_stream.write_bit(bit))
}

// Serializes an internal node: writes marker bit '0' followed by both children
// Uses Result chaining (and_then) to serialize left child then right child
// If a child doesn't exist, treats it as a successful no-op
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

// Converts a Huffman tree to a string of '0' and '1' characters
// Used primarily for testing and debugging purposes
// Serializes the tree to bits, then converts those bits to a readable string
pub fn serialize_tree(tree: &HuffmanNode) -> String {
    let mut output = Vec::new();
    let mut bit_stream = OutputBitStream::new(&mut output);

    serialize_tree_to_bits(tree, &mut bit_stream).expect("Failed to serialize tree to bits");

    let bits_written = count_tree_bits(tree);

    bit_stream.flush().expect("Failed to flush bit stream");

    bits_to_string(&output, bits_written)
}

// Calculates how many bits are needed to represent the tree structure
// Leaf nodes need 9 bits (1 marker + 8 symbol bits)
// Internal nodes need 1 bit for marker + bits for both children
// Uses recursion to count bits for the entire tree
fn count_tree_bits(tree: &HuffmanNode) -> usize {
    if tree.is_leaf() {
        9
    } else {
        1 + tree.left_child().map_or(0, count_tree_bits)
            + tree.right_child().map_or(0, count_tree_bits)
    }
}

// Converts raw bytes to a string representation of bits
// Takes the first num_bits from the byte array and converts each to '0' or '1'
// Uses iterator chaining: for each byte, extracts its 8 bits, then takes only what's needed
// Example: [0b10110000] with num_bits=3 becomes "101"
fn bits_to_string(bytes: &[u8], num_bits: usize) -> String {
    bytes
        .iter()
        .flat_map(|&byte| (0..8).rev().map(move |i| (byte >> i) & 1))
        .take(num_bits)
        .map(|bit| if bit == 1 { '1' } else { '0' })
        .collect()
}
