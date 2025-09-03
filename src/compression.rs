use crate::{
    code_extraction::extract_huffman_codes, frequency_map::ByteFrequencyMap,
    output_bit_stream::OutputBitStream, tree_construction::build_huffman_tree,
    tree_serialization::serialize_tree_to_bits,
};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

/// Count byte frequencies by reading from a stream
fn count_frequencies_from_stream<R: Read>(
    mut reader: R,
) -> std::io::Result<(ByteFrequencyMap, usize)> {
    let mut frequency_map = HashMap::new();
    let mut total_bytes = 0;
    let mut buffer = [0u8; 8192]; // 8KB buffer for efficient reading

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of stream
        }

        total_bytes += bytes_read;

        for &byte in &buffer[..bytes_read] {
            *frequency_map.entry(byte).or_insert(0) += 1;
        }
    }

    Ok((frequency_map, total_bytes))
}

fn encode_input_stream<R: Read, W: std::io::Write>(
    mut input_reader: R,
    codes: &HashMap<u8, String>,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    let mut buffer = [0u8; 8192]; // 8KB buffer for efficient reading

    loop {
        let bytes_read = input_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of stream
        }

        for &byte in &buffer[..bytes_read] {
            if let Some(code) = codes.get(&byte) {
                for bit_char in code.chars() {
                    let bit = if bit_char == '1' { 1 } else { 0 };
                    bit_stream.write_bit(bit)?;
                }
            }
        }
    }

    Ok(())
}

pub fn compress<R: Read + Seek, W: Write>(
    mut input_reader: R,
    output_stream: &mut W,
) -> std::io::Result<()> {
    // First pass: count frequencies
    let (frequency_map, total_bytes) = count_frequencies_from_stream(&mut input_reader)?;

    // Seek back to beginning for second pass
    input_reader.seek(SeekFrom::Start(0))?;

    // Build Huffman tree from frequencies
    let tree = build_huffman_tree(&frequency_map);
    let codes = extract_huffman_codes(&tree);

    // Write original length as header
    let original_length = total_bytes as u32;
    output_stream.write_all(&original_length.to_le_bytes())?;

    // Initialize bit stream for compressed output
    let mut bit_stream = OutputBitStream::new(output_stream);

    // Serialize tree structure to bit stream
    serialize_tree_to_bits(&tree, &mut bit_stream)?;

    // Second pass: encode input data
    encode_input_stream(&mut input_reader, &codes, &mut bit_stream)?;

    // Ensure all bits are written
    bit_stream.flush()?;

    Ok(())
}
