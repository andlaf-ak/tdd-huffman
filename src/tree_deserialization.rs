use crate::constants::BITS_PER_BYTE;
use crate::input_bit_stream::InputBitStream;
use crate::tree_construction::HuffmanNode;

const LEAF_NODE_BIT: u8 = 1;
const INTERNAL_NODE_BIT: u8 = 0;

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

fn deserialize_leaf_node<R: std::io::Read>(
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<HuffmanNode> {
    let symbol = read_symbol_from_bits(bit_stream)?;
    Ok(HuffmanNode::new_leaf(symbol, 1))
}

fn deserialize_internal_node<R: std::io::Read>(
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<HuffmanNode> {
    let left_child = deserialize_tree(bit_stream)?;
    let right_child = deserialize_tree(bit_stream)?;
    Ok(HuffmanNode::new_internal(left_child, right_child))
}

fn read_symbol_from_bits<R: std::io::Read>(
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<u8> {
    let mut symbol = 0u8;
    for _ in 0..BITS_PER_BYTE {
        let bit = bit_stream.read_bit()?;
        symbol = (symbol << 1) | bit;
    }
    Ok(symbol)
}
