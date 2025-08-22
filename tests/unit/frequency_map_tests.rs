use tdd_huffman::count_byte_frequencies;

#[test]
fn count_single_byte_occurrence() {
    let input = [65u8];
    let frequencies = count_byte_frequencies(&input);
    assert_eq!(frequencies.get(&65), Some(&1));
}

#[test]
fn count_multiple_occurrences_of_same_byte() {
    let input = [65u8, 65u8, 65u8];
    let frequencies = count_byte_frequencies(&input);
    assert_eq!(frequencies.get(&65), Some(&3));
}

#[test]
fn count_different_bytes_with_different_frequencies() {
    let input = [65u8, 66u8, 65u8, 67u8, 66u8, 66u8];
    let frequencies = count_byte_frequencies(&input);
    assert_eq!(frequencies.get(&65), Some(&2)); // 'A' appears 2 times
    assert_eq!(frequencies.get(&66), Some(&3)); // 'B' appears 3 times
    assert_eq!(frequencies.get(&67), Some(&1)); // 'C' appears 1 time
}

#[test]
fn handle_empty_input_gracefully() {
    let input: &[u8] = &[];
    let frequencies = count_byte_frequencies(input);

    // Basic requirements that should pass
    assert_eq!(frequencies.len(), 0);
    assert!(frequencies.is_empty());

    // New TDD requirement: empty input should result in exactly zero total count
    // This is a reasonable requirement for frequency analysis
    let total_count: usize = frequencies.values().sum();
    assert_eq!(total_count, 0);

    // New requirement: empty input handling should be documented in function behavior
    // We want to ensure that calling the function with empty input is safe and defined
    assert_eq!(frequencies.get(&0), None);
    assert_eq!(frequencies.get(&255), None);

    // Additional requirement: consecutive calls with empty input should be consistent
    let frequencies2 = count_byte_frequencies(&[]);
    assert_eq!(frequencies, frequencies2);
}
