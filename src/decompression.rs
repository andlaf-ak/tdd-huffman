use crate::input_bit_stream::InputBitStream;
use crate::tree_construction::HuffmanNode;
use crate::tree_deserialization::deserialize_tree;
use std::io::{Read, Write};

const LEFT_BIT: u8 = 0;
const RIGHT_BIT: u8 = 1;

// Decompresses Huffman-encoded data back to original form
// Step 1: Read the 4-byte header containing original file size
// Step 2: Deserialize the Huffman tree from the bit stream
// Step 3: Use the tree to decode the compressed data back to original bytes
// Uses Result chaining (and_then) to handle errors at each step
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

// Decodes compressed data using the Huffman tree
// Handles two cases: single symbol trees (where all data is the same character)
// and multi-symbol trees (where we traverse the tree using bits to find symbols)
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

// Handles the special case where all input was the same character
// Simply repeats the single symbol for the required number of times
// Uses repeat_n to generate the required number of copies efficiently
fn decode_single_symbol_tree<W: Write>(
    tree: &HuffmanNode,
    output_stream: &mut W,
    output_length: usize,
) -> std::io::Result<()> {
    let symbol = tree.symbol().expect("Leaf node must have a symbol");
    let symbols = std::iter::repeat_n(symbol, output_length).collect::<Vec<_>>();
    output_stream.write_all(&symbols)
}

// Creates an iterator that decodes the specified number of symbols
// Each iteration reads bits from the stream and traverses the Huffman tree
// until it reaches a leaf node, then returns the symbol at that leaf
// Returns an iterator of Results to handle any IO errors during decoding
fn decode_symbols<'a, R: Read>(
    tree: &'a HuffmanNode,
    bit_stream: &'a mut InputBitStream<R>,
    count: usize,
) -> impl Iterator<Item = std::io::Result<u8>> + 'a {
    (0..count).map(move |_| decode_next_symbol(tree, bit_stream))
}

// Decodes data from a tree with multiple different symbols
// Uses the decode_symbols iterator to get all the decoded bytes
// Collects them into a vector and writes the entire result at once
// This is more efficient than writing each symbol individually
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

// Decodes a single symbol by traversing the Huffman tree
// Starts at the root and follows the tree based on bits from the input:
// - 0 bit = go to left child
// - 1 bit = go to right child
// Continues until reaching a leaf node, then returns the symbol at that leaf
// Uses successors to generate a sequence of tree nodes based on input bits
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
