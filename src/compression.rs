use crate::{
    code_extraction::extract_huffman_codes,
    frequency_map::count_byte_frequencies,
    output_bit_stream::OutputBitStream,
    tree_construction::build_huffman_tree,
    tree_serialization::{serialize_tree, serialize_tree_to_bits},
};
use std::collections::HashMap;
use std::io::{Read, Write};

fn encode_input_stream<W: std::io::Write>(
    input_bytes: &[u8],
    codes: &HashMap<u8, String>,
    bit_stream: &mut OutputBitStream<W>,
) -> std::io::Result<()> {
    for &byte in input_bytes {
        if let Some(code) = codes.get(&byte) {
            for bit_char in code.chars() {
                let bit = if bit_char == '1' { 1 } else { 0 };
                bit_stream.write_bit(bit)?;
            }
        }
    }
    Ok(())
}

pub fn compress<R: Read, W: Write>(
    mut input_reader: R,
    output_stream: &mut W,
) -> std::io::Result<()> {
    // Read all input data into memory first to analyze frequencies
    let mut input_bytes = Vec::new();
    input_reader.read_to_end(&mut input_bytes)?;

    // Build Huffman tree from input data
    let frequency_map = count_byte_frequencies(&input_bytes);
    let tree = build_huffman_tree(&frequency_map);
    let codes = extract_huffman_codes(&tree);

    // Write original length as header
    let original_length = input_bytes.len() as u32;
    output_stream.write_all(&original_length.to_le_bytes())?;
    
    // Initialize bit stream for compressed output
    let mut bit_stream = OutputBitStream::new(output_stream);

    // Serialize tree structure to bit stream
    serialize_tree_to_bits(&tree, &mut bit_stream)?;

    // Encode input data using Huffman codes
    encode_input_stream(&input_bytes, &codes, &mut bit_stream)?;

    // Ensure all bits are written
    bit_stream.flush()?;

    Ok(())
}

/// Statistics about compression operation
#[derive(Debug)]
pub struct CompressionStats {
    pub original_bits: usize,
    pub compressed_bits: usize,
    pub compression_ratio: f64,
    pub frequency_map: HashMap<u8, usize>,
    pub huffman_codes: HashMap<u8, String>,
    pub serialized_tree: String,
}

/// Compress with detailed statistics - mainly for testing purposes
pub fn compress_with_stats<R: Read, W: Write>(
    mut input_reader: R,
    output_stream: &mut W,
) -> std::io::Result<CompressionStats> {
    // Read all input data into memory first to analyze frequencies
    let mut input_bytes = Vec::new();
    input_reader.read_to_end(&mut input_bytes)?;
    
    let original_bits = input_bytes.len() * 8;

    // Build Huffman tree from input data
    let frequency_map = count_byte_frequencies(&input_bytes);
    let tree = build_huffman_tree(&frequency_map);
    let codes = extract_huffman_codes(&tree);
    let serialized_tree = serialize_tree(&tree);

    // Track bytes written for statistics
    let mut bytes_written = 0;

    // Write original length as header
    let original_length = input_bytes.len() as u32;
    let header_bytes = original_length.to_le_bytes();
    output_stream.write_all(&header_bytes)?;
    bytes_written += header_bytes.len();
    
    // Use a counting wrapper to track compressed output
    let mut counting_writer = CountingWriter::new(output_stream);
    let mut bit_stream = OutputBitStream::new(&mut counting_writer);

    // Serialize tree structure to bit stream
    serialize_tree_to_bits(&tree, &mut bit_stream)?;

    // Encode input data using Huffman codes
    encode_input_stream(&input_bytes, &codes, &mut bit_stream)?;

    // Ensure all bits are written
    bit_stream.flush()?;

    // Calculate total compressed bits
    let compressed_bits = (bytes_written + counting_writer.bytes_written()) * 8;
    let compression_ratio = if original_bits > 0 {
        compressed_bits as f64 / original_bits as f64
    } else {
        0.0
    };

    Ok(CompressionStats {
        original_bits,
        compressed_bits,
        compression_ratio,
        frequency_map,
        huffman_codes: codes,
        serialized_tree,
    })
}

/// A writer wrapper that tracks the number of bytes written
struct CountingWriter<W> {
    writer: W,
    bytes_written: usize,
}

impl<W: Write> CountingWriter<W> {
    fn new(writer: W) -> Self {
        Self {
            writer,
            bytes_written: 0,
        }
    }

    fn bytes_written(&self) -> usize {
        self.bytes_written
    }
}

impl<W: Write> Write for CountingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes_written = self.writer.write(buf)?;
        self.bytes_written += bytes_written;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
