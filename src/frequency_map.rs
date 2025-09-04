use std::collections::HashMap;
use std::io::Read;

pub type ByteFrequencyMap = HashMap<u8, usize>;

// Updates the frequency count for a single byte in the map
// If the byte is already in the map, increments its count by 1
// If the byte is not in the map, adds it with a count of 1
// Returns the updated map (this is a pure function - doesn't modify the original)
fn update_frequency_map(mut frequency_map: ByteFrequencyMap, byte: u8) -> ByteFrequencyMap {
    *frequency_map.entry(byte).or_insert(0) += 1;
    frequency_map
}

// Counts how often each byte value appears in the input stream
// Reads the input in 8KB chunks for memory efficiency
// Uses an iterator pattern: reads chunks until no more data
// For each chunk, counts occurrences of each byte value
// Returns both the frequency map and total number of bytes processed
// The frequency map is built by folding over all bytes, accumulating counts
pub fn count_frequencies<R: Read>(mut reader: R) -> std::io::Result<(ByteFrequencyMap, usize)> {
    let mut buffer = [0u8; 8192];
    let mut total_bytes = 0;

    std::iter::from_fn(|| match reader.read(&mut buffer) {
        Ok(0) => None,
        Ok(bytes_read) => {
            total_bytes += bytes_read;
            Some(buffer[..bytes_read].to_vec())
        }
        Err(_) => None,
    })
    .flatten()
    .try_fold(HashMap::new(), |acc, byte| {
        Ok(update_frequency_map(acc, byte))
    })
    .map(|frequency_map| (frequency_map, total_bytes))
}
