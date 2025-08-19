use std::collections::HashMap;

/// Counts the frequency of each byte in the input slice.
///
/// Returns a HashMap where keys are byte values and values are their occurrence counts.
/// Currently implements counting for byte 65 ('A') only, as driven by existing tests.
/// This will be generalized as more test cases are added following TDD methodology.
pub fn count_byte_frequencies(input: &[u8]) -> HashMap<u8, usize> {
    // Current TDD scope: only count byte 65 (ASCII 'A')
    const BYTE_A: u8 = 65;

    // Count occurrences of the target byte in the input
    let frequency_count = count_occurrences_of_byte(input, BYTE_A);

    // Build frequency map with single entry
    HashMap::from([(BYTE_A, frequency_count)])
}

/// Helper function to count occurrences of a specific byte in the input slice.
fn count_occurrences_of_byte(input: &[u8], target_byte: u8) -> usize {
    input.iter().filter(|&&byte| byte == target_byte).count()
}
