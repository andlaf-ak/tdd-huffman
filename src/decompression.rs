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
    let mut reader = input_reader;
    let mut header_bytes = [0u8; 4];
    reader
        .read_exact(&mut header_bytes)
        .map(|_| u32::from_le_bytes(header_bytes) as usize)
        .and_then(|original_length| {
            let mut bit_stream = InputBitStream::new(reader);
            deserialize_tree(&mut bit_stream).and_then(|tree| {
                decode_compressed_data(&tree, &mut bit_stream, output_stream, original_length)
            })
        })
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
    let symbols = std::iter::repeat_n(symbol, output_length).collect::<Vec<_>>();
    output_stream.write_all(&symbols)
}

fn decode_symbols<'a, R: Read>(
    tree: &'a HuffmanNode,
    bit_stream: &'a mut InputBitStream<R>,
    count: usize,
) -> impl Iterator<Item = std::io::Result<u8>> + 'a {
    (0..count).map(move |_| decode_next_symbol(tree, bit_stream))
}

fn decode_multi_symbol_tree<R: Read, W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut InputBitStream<R>,
    output_stream: &mut W,
    output_length: usize,
) -> std::io::Result<()> {
    let symbols: Result<Vec<_>, _> = decode_symbols(tree, bit_stream, output_length).collect();
    let symbols = symbols?;
    output_stream.write_all(&symbols)
}

fn decode_next_symbol<R: Read>(
    tree: &HuffmanNode,
    bit_stream: &mut InputBitStream<R>,
) -> std::io::Result<u8> {
    std::iter::successors(Some(Ok(tree)), |node_result| {
        node_result.as_ref().ok().and_then(|node| {
            if node.is_leaf() {
                None
            } else {
                Some(bit_stream.read_bit().and_then(|bit| {
                    match bit {
                        LEFT_BIT => Ok(node
                            .left_child()
                            .expect("Internal node must have left child")),
                        RIGHT_BIT => Ok(node
                            .right_child()
                            .expect("Internal node must have right child")),
                        _ => Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Invalid bit value: {bit}"),
                        )),
                    }
                }))
            }
        })
    })
    .last()
    .unwrap()
    .map(|node| node.symbol().expect("Leaf node must have a symbol"))
}
