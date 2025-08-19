use std::collections::HashMap;

/// Type alias for byte frequency maps to improve readability.
pub type ByteFrequencyMap = HashMap<u8, usize>;

/// Counts the frequency of each byte in the input slice.
///
/// Returns a HashMap where keys are byte values and values are their occurrence counts.
/// Only includes entries for bytes that actually appear in the input.
///
/// # Examples
///
/// ```
/// use tdd_huffman::count_byte_frequencies;
///
/// // Single byte
/// let input = [65u8];
/// let frequencies = count_byte_frequencies(&input);
/// assert_eq!(frequencies.get(&65), Some(&1));
///
/// // Multiple different bytes
/// let input = [65u8, 66u8, 65u8, 67u8];
/// let frequencies = count_byte_frequencies(&input);
/// assert_eq!(frequencies.get(&65), Some(&2)); // 'A' appears twice
/// assert_eq!(frequencies.get(&66), Some(&1)); // 'B' appears once
/// assert_eq!(frequencies.get(&67), Some(&1)); // 'C' appears once
/// ```
pub fn count_byte_frequencies(input: &[u8]) -> ByteFrequencyMap {
    input
        .iter()
        .fold(ByteFrequencyMap::new(), |mut frequency_map, &byte| {
            *frequency_map.entry(byte).or_insert(0) += 1;
            frequency_map
        })
}
