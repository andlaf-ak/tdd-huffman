use crate::input_bit_stream::InputBitStream;
use crate::tree_construction::HuffmanNode;
use std::io::Write;

const LEFT_BIT: u8 = 0;
const RIGHT_BIT: u8 = 1;

pub fn decompress<R: std::io::Read, W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut InputBitStream<R>,
    output_stream: &mut W,
    original_length: usize,
) -> std::io::Result<()> {
    if tree.is_leaf() {
        decompress_single_symbol_tree(tree, output_stream, original_length)
    } else {
        decompress_multi_symbol_tree(tree, bit_stream, output_stream, original_length)
    }
}

fn decompress_single_symbol_tree<W: Write>(
    tree: &HuffmanNode,
    output_stream: &mut W,
    output_length: usize,
) -> std::io::Result<()> {
    let symbol = tree.symbol().expect("Leaf node must have a symbol");
    for _ in 0..output_length {
        output_stream.write_all(&[symbol])?;
    }
    Ok(())
}

fn decompress_multi_symbol_tree<R: std::io::Read, W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut InputBitStream<R>,
    output_stream: &mut W,
    output_length: usize,
) -> std::io::Result<()> {
    for _ in 0..output_length {
        let symbol = decode_next_symbol(tree, bit_stream)?;
        output_stream.write_all(&[symbol])?;
    }
    Ok(())
}

fn decode_next_symbol<R: std::io::Read>(
    tree: &HuffmanNode,
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<u8> {
    let mut current_node = tree;
    
    while !current_node.is_leaf() {
        let bit = bit_stream.read_bit()?;
        current_node = match bit {
            LEFT_BIT => current_node.left_child().expect("Internal node must have left child"),
            RIGHT_BIT => current_node.right_child().expect("Internal node must have right child"),
            _ => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid bit value: {}", bit),
            )),
        };
    }
    
    Ok(current_node.symbol().expect("Leaf node must have a symbol"))
}
