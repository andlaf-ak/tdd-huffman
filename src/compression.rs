use crate::{
    code_extraction::extract_huffman_codes, 
    frequency_map::count_frequencies,
    output_bit_stream::OutputBitStream, tree_construction::build_huffman_tree,
    tree_serialization::serialize_tree_to_bits,
};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

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
    let (frequency_map, total_bytes) = count_frequencies(&mut input_reader)?;

    input_reader.seek(SeekFrom::Start(0))?;

    let tree = build_huffman_tree(&frequency_map);
    let codes = extract_huffman_codes(&tree);

    let original_length = total_bytes as u32;
    output_stream.write_all(&original_length.to_le_bytes())?;

    let mut bit_stream = OutputBitStream::new(output_stream);

    serialize_tree_to_bits(&tree, &mut bit_stream)?;

    encode_input_stream(&mut input_reader, &codes, &mut bit_stream)?;

    bit_stream.flush()?;

    Ok(())
}
