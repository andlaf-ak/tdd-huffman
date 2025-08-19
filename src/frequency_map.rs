use std::collections::HashMap;

/// Counts the frequency of each byte in the input slice.
///
/// Returns a HashMap where keys are byte values and values are their occurrence counts.
/// Implements full frequency counting for all bytes as driven by current tests.
pub fn count_byte_frequencies(input: &[u8]) -> HashMap<u8, usize> {
    let mut frequency_map = HashMap::new();
    
    // Count each byte in the input
    for &byte in input {
        *frequency_map.entry(byte).or_insert(0) += 1;
    }
    
    frequency_map
}
