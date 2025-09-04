use std::collections::HashMap;
use std::io::Read;

pub type ByteFrequencyMap = HashMap<u8, usize>;

fn update_frequency_map(mut frequency_map: ByteFrequencyMap, byte: u8) -> ByteFrequencyMap {
    *frequency_map.entry(byte).or_insert(0) += 1;
    frequency_map
}

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
