pub fn assert_original_length_in_header(compressed_data: &[u8], expected_length: usize) {
    assert!(
        compressed_data.len() >= 4,
        "Compressed data should be at least 4 bytes to include length header"
    );

    let length_bytes = &compressed_data[0..4];
    let stored_length = u32::from_le_bytes([
        length_bytes[0],
        length_bytes[1],
        length_bytes[2],
        length_bytes[3],
    ]) as usize;

    assert_eq!(
        stored_length, expected_length,
        "First 4 bytes should contain original data length: expected {}, got {}",
        expected_length, stored_length
    );
}
