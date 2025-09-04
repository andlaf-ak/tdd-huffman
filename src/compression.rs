use crate::{
    code_extraction::extract_huffman_codes, frequency_map::count_frequencies,
    output_bit_stream::OutputBitStream, tree_construction::build_huffman_tree,
    tree_serialization::serialize_tree_to_bits,
};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

// Converts a single byte to its Huffman code bits
// Takes a byte value and looks it up in the codes table to get its binary representation
// For example: if byte 'A' has code "101", this returns [1, 0, 1]
// Returns None if the byte is not found in the codes table
fn encode_byte(byte: u8, codes: &HashMap<u8, String>) -> Option<Vec<u8>> {
    codes.get(&byte).map(|code| {
        code.chars()
            .map(|bit_char| if bit_char == '1' { 1 } else { 0 })
            .collect()
    })
}

// Encodes a stream of bytes into Huffman-coded bits
// Takes an iterator of bytes (each wrapped in Result for error handling)
// For each successful byte, looks up its Huffman code and converts to individual bits
// Flattens all the bits into a single stream - so "AB" might become [1,0,1,1,1,0,0]
// Preserves any IO errors that occurred while reading the input bytes
fn encode_bytes<I: IntoIterator<Item = std::io::Result<u8>>>(
    bytes: I,
    codes: &HashMap<u8, String>,
) -> impl Iterator<Item = std::io::Result<u8>> + use<'_, I> {
    bytes
        .into_iter()
        .flat_map(move |byte_result| match byte_result {
            Ok(byte) => encode_byte(byte, codes)
                .map(|bits| bits.into_iter().map(Ok).collect::<Vec<_>>())
                .unwrap_or_default(),
            Err(e) => vec![Err(e)],
        })
}

// Reads input in chunks and encodes each byte using Huffman codes
// Uses a buffer to read 8KB chunks at a time for efficiency
// For each chunk, converts bytes to Huffman bits and writes them to the bit stream
// This avoids loading the entire file into memory while still using iterator patterns
fn encode_input_stream<R: Read, W: std::io::Write>(
    mut input_reader: R,
    codes: &HashMap<u8, String>,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = input_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let byte_iter = buffer[..bytes_read].iter().map(|&b| Ok(b));
        for bit_result in encode_bytes(byte_iter, codes) {
            let bit = bit_result?;
            bit_stream.write_bit(bit)?;
        }
    }

    Ok(())
}

// Compresses input data using Huffman coding algorithm
// Step 1: Count how often each byte appears in the input
// Step 2: Build a Huffman tree based on these frequencies
// Step 3: Extract binary codes for each byte from the tree
// Step 4: Write the original file size and tree structure to output
// Step 5: Encode the actual data using the generated codes
// Uses Result chaining (and_then) to handle errors gracefully at each step
pub fn compress<R: Read + Seek, W: Write>(
    mut input_reader: R,
    output_stream: &mut W,
) -> std::io::Result<()> {
    count_frequencies(&mut input_reader)
        .and_then(|(frequency_map, total_bytes)| {
            input_reader.seek(SeekFrom::Start(0))?;

            let tree = build_huffman_tree(&frequency_map);
            let codes = extract_huffman_codes(&tree);

            Ok((tree, codes, total_bytes))
        })
        .and_then(|(tree, codes, total_bytes)| {
            let original_length = total_bytes as u32;
            output_stream.write_all(&original_length.to_le_bytes())?;

            let mut bit_stream = OutputBitStream::new(output_stream);

            serialize_tree_to_bits(&tree, &mut bit_stream)
                .and_then(|_| encode_input_stream(&mut input_reader, &codes, &mut bit_stream))
                .and_then(|_| bit_stream.flush())
        })
}
