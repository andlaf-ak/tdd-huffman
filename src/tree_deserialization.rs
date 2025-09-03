use crate::input_bit_stream::InputBitStream;
use crate::tree_construction::HuffmanNode;

pub fn deserialize_tree<R: std::io::Read>(
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<HuffmanNode> {
    // Read first bit to determine if this is a leaf or internal node
    let node_type_bit = bit_stream.read_bit()?;
    
    if node_type_bit == 1 {
        // Leaf node: read 8 bits for the symbol
        let mut symbol = 0u8;
        for _ in 0..8 {
            let bit = bit_stream.read_bit()?;
            symbol = (symbol << 1) | bit;
        }
        Ok(HuffmanNode::new_leaf(symbol, 1))
    } else {
        // Internal node (not implemented yet)
        todo!("Internal node deserialization not implemented yet")
    }
}
