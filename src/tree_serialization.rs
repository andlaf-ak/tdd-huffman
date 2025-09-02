use crate::tree_construction::HuffmanNode;
use crate::output_bit_stream::OutputBitStream;
use std::io::Write;

/// Core bit-based tree serialization function
/// This is the primary implementation that writes directly to a bit stream
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
    // Write '1' to indicate this is a leaf node
    bit_stream.write_bit(1)?;
    
    // Write the 8-bit symbol
    let symbol = tree.symbol().expect("Leaf node must have a symbol");
    for i in (0..8).rev() {
        let bit = (symbol >> i) & 1;
        bit_stream.write_bit(bit)?;
    }
    
    Ok(())
}

fn serialize_internal_to_bits<W: Write>(
    tree: &HuffmanNode,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    // Write '0' to indicate this is an internal node
    bit_stream.write_bit(0)?;
    
    // Recursively serialize left and right children
    if let Some(left_child) = tree.left_child() {
        serialize_tree_to_bits(left_child, bit_stream)?;
    }
    
    if let Some(right_child) = tree.right_child() {
        serialize_tree_to_bits(right_child, bit_stream)?;
    }
    
    Ok(())
}

/// String-based tree serialization function (refactored to use bit-based implementation)
/// This function now calls the bit-based version and converts the result to a string
pub fn serialize_tree(tree: &HuffmanNode) -> String {
    let mut output = Vec::new();
    let mut bit_stream = OutputBitStream::new(&mut output);
    
    serialize_tree_to_bits(tree, &mut bit_stream)
        .expect("Failed to serialize tree to bits");
    
    // Count how many bits we actually wrote
    let bits_written = count_tree_bits(tree);
    
    bit_stream.flush()
        .expect("Failed to flush bit stream");
    
    // Convert the bit stream output back to a string representation
    bits_to_string(&output, bits_written)
}

/// Count the total number of bits needed to serialize a tree
fn count_tree_bits(tree: &HuffmanNode) -> usize {
    if tree.is_leaf() {
        9 // 1 bit (leaf marker) + 8 bits (symbol)
    } else {
        1 + // 1 bit (internal marker)
        tree.left_child().map_or(0, count_tree_bits) +
        tree.right_child().map_or(0, count_tree_bits)
    }
}

/// Helper function to convert bit stream output to string representation
fn bits_to_string(bytes: &[u8], num_bits: usize) -> String {
    let mut result = String::new();
    let mut bits_read = 0;
    
    for &byte in bytes {
        for i in (0..8).rev() {
            if bits_read >= num_bits {
                break;
            }
            let bit = (byte >> i) & 1;
            result.push(if bit == 1 { '1' } else { '0' });
            bits_read += 1;
        }
        if bits_read >= num_bits {
            break;
        }
    }
    
    result
}
