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
