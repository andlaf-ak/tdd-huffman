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
