use std::collections::HashMap;
use std::io::Read;

pub type ByteFrequencyMap = HashMap<u8, usize>;

pub fn count_frequencies<R: Read>(mut reader: R) -> std::io::Result<(ByteFrequencyMap, usize)> {
    let mut frequency_map = HashMap::new();
    let mut total_bytes = 0;
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        total_bytes += bytes_read;

        for &byte in &buffer[..bytes_read] {
            *frequency_map.entry(byte).or_insert(0) += 1;
        }
    }

    Ok((frequency_map, total_bytes))
}
