use crate::input_bit_stream::InputBitStream;
use crate::tree_construction::HuffmanNode;
use crate::tree_deserialization::deserialize_tree;
use std::io::{Read, Write};

const LEFT_BIT: u8 = 0;
const RIGHT_BIT: u8 = 1;

pub fn decompress<R: Read, W: Write>(
    input_reader: R,
    output_stream: &mut W,
) -> std::io::Result<()> {
    // Read the original data length from the 4-byte header
    let mut reader = input_reader;
    let mut header_bytes = [0u8; 4];
    reader.read_exact(&mut header_bytes)?;
    let original_length = u32::from_le_bytes(header_bytes) as usize;

    // Create bit stream for the remaining data
    let mut bit_stream = InputBitStream::new(reader);

    // Deserialize the tree from the bit stream
    let tree = deserialize_tree(&mut bit_stream)?;

    // Decode the compressed data
    decode_compressed_data(&tree, &mut bit_stream, output_stream, original_length)
}

pub fn decode_compressed_data<R: Read, W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut InputBitStream<R>,
    output_stream: &mut W,
    original_length: usize,
) -> std::io::Result<()> {
    if tree.is_leaf() {
        decode_single_symbol_tree(tree, output_stream, original_length)
    } else {
        decode_multi_symbol_tree(tree, bit_stream, output_stream, original_length)
    }
}

fn decode_single_symbol_tree<W: Write>(
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

fn decode_multi_symbol_tree<R: Read, W: Write>(
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

fn decode_next_symbol<R: Read>(
    tree: &HuffmanNode,
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<u8> {
    let mut current_node = tree;

    while !current_node.is_leaf() {
        let bit = bit_stream.read_bit()?;
        current_node = match bit {
            LEFT_BIT => current_node
                .left_child()
                .expect("Internal node must have left child"),
            RIGHT_BIT => current_node
                .right_child()
                .expect("Internal node must have right child"),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid bit value: {}", bit),
                ))
            }
        };
    }

    Ok(current_node.symbol().expect("Leaf node must have a symbol"))
}
