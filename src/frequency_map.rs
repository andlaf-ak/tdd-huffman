use std::collections::HashMap;

pub type ByteFrequencyMap = HashMap<u8, usize>;

pub fn count_byte_frequencies(input: &[u8]) -> ByteFrequencyMap {
    input
        .iter()
        .fold(ByteFrequencyMap::new(), |mut frequency_map, &byte| {
            *frequency_map.entry(byte).or_insert(0) += 1;
            frequency_map
        })
}
