use tdd_huffman::select_nodes;

#[test]
fn select_single_node_when_only_one_symbol_exists() {
    let input = [(65u8, 1usize)]; // Single symbol 'A' with frequency 1
    let selected_nodes = select_nodes(&input);

    assert_eq!(selected_nodes.len(), 1);
    // The single node should contain the symbol and its frequency
    assert!(selected_nodes.contains(&(65u8, 1usize)));
}

#[test]
fn select_two_nodes_with_lowest_frequencies_from_multiple_options() {
    let input = [
        (65u8, 5usize),
        (66u8, 2usize),
        (67u8, 8usize),
        (68u8, 3usize),
    ];
    // 'A':5, 'B':2, 'C':8, 'D':3 - should select B(2) and D(3) as lowest frequencies
    let selected_nodes = select_nodes(&input);

    assert_eq!(selected_nodes.len(), 2);
    // Should select the two nodes with lowest frequencies: B(2) and D(3)
    assert!(selected_nodes.contains(&(66u8, 2usize))); // 'B' with frequency 2
    assert!(selected_nodes.contains(&(68u8, 3usize))); // 'D' with frequency 3
                                                       // Should NOT contain the higher frequency nodes
    assert!(!selected_nodes.contains(&(65u8, 5usize))); // 'A' with frequency 5
    assert!(!selected_nodes.contains(&(67u8, 8usize))); // 'C' with frequency 8
}

#[test]
fn select_nodes_with_tie_breaking_when_frequencies_are_equal() {
    let input = [
        (67u8, 3usize), // 'C' with frequency 3
        (65u8, 3usize), // 'A' with frequency 3 (same as C)
        (66u8, 5usize), // 'B' with frequency 5
        (68u8, 3usize), // 'D' with frequency 3 (same as A and C)
    ];
    // When frequencies are tied, should break ties by symbol value (ASCII order)
    // Expected: select A(3) and C(3) as they have lowest frequencies and lowest symbol values
    let selected_nodes = select_nodes(&input);

    assert_eq!(selected_nodes.len(), 2);
    // Should select A(3) and C(3) - lowest frequency, with ties broken by symbol order
    assert_eq!(selected_nodes, vec![(65u8, 3usize), (67u8, 3usize)]); // A then C in order
                                                                      // Should NOT contain the other nodes
    assert!(!selected_nodes.contains(&(66u8, 5usize))); // 'B' with frequency 5
    assert!(!selected_nodes.contains(&(68u8, 3usize))); // 'D' with frequency 3 (loses tie to A and C)
}
