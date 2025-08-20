use tdd_huffman::select_nodes;

#[test]
fn select_single_node_when_only_one_symbol_exists() {
    let input = [(65u8, 1usize)]; // Single symbol 'A' with frequency 1
    let selected_nodes = select_nodes(&input);
    
    assert_eq!(selected_nodes.len(), 1);
    // The single node should contain the symbol and its frequency
    assert!(selected_nodes.contains(&(65u8, 1usize)));
}
