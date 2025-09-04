use crate::constants::BITS_PER_BYTE;
use crate::input_bit_stream::InputBitStream;
use crate::tree_construction::HuffmanNode;

const LEAF_NODE_BIT: u8 = 1;
const INTERNAL_NODE_BIT: u8 = 0;

// Reconstructs a Huffman tree from its binary representation
// Reads the first bit to determine node type:
// - '1' bit means leaf node (followed by 8 symbol bits)
// - '0' bit means internal node (followed by left and right child trees)
// This reverses the process done by serialize_tree_to_bits
pub fn deserialize_tree<R: std::io::Read>(
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<HuffmanNode> {
    let node_type_bit = bit_stream.read_bit()?;

    match node_type_bit {
        LEAF_NODE_BIT => deserialize_leaf_node(bit_stream),
        INTERNAL_NODE_BIT => deserialize_internal_node(bit_stream),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid node type bit: {node_type_bit}"),
        )),
    }
}

// Reconstructs a leaf node from the bit stream
// Reads the next 8 bits to get the symbol value
// Creates a new leaf node with that symbol (frequency set to 1 since it's not needed for decompression)
fn deserialize_leaf_node<R: std::io::Read>(
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<HuffmanNode> {
    let symbol = read_symbol_from_bits(bit_stream)?;
    Ok(HuffmanNode::new_leaf(symbol, 1))
}

// Reconstructs an internal node from the bit stream
// Recursively deserializes the left child tree, then the right child tree
// Uses Result chaining (and_then) to handle errors from either child
// Combines both children into a new internal node
fn deserialize_internal_node<R: std::io::Read>(
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<HuffmanNode> {
    deserialize_tree(bit_stream).and_then(|left_child| {
        deserialize_tree(bit_stream)
            .map(|right_child| HuffmanNode::new_internal(left_child, right_child))
    })
}

// Reads exactly 8 bits from the stream and combines them into a byte
// Reads bits from most significant to least significant position
// Uses try_fold to accumulate bits: starts with 0, shifts left and adds each new bit
// Example: reading bits 1,0,1,1,0,0,1,0 produces byte value 178
fn read_symbol_from_bits<R: std::io::Read>(
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<u8> {
    (0..BITS_PER_BYTE).try_fold(0u8, |acc, _| {
        bit_stream.read_bit().map(|bit| (acc << 1) | bit)
    })
}
