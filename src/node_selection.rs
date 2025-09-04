pub type SymbolFrequency = (u8, usize);
pub type NodeCollection = Vec<SymbolFrequency>;

// Selects nodes for Huffman tree construction based on frequency data
// For single symbols: returns the symbol as-is (special case)
// For multiple symbols: finds the two symbols with lowest frequencies
// This implements the core Huffman algorithm: always combine least frequent items
pub fn select_nodes(frequency_data: &[SymbolFrequency]) -> NodeCollection {
    if frequency_data.is_empty() {
        return Vec::new();
    }

    const HUFFMAN_PAIR_COUNT: usize = 2;

    if is_single_symbol_case(frequency_data) {
        create_initial_nodes(frequency_data)
    } else {
        select_lowest_frequency_nodes(frequency_data, HUFFMAN_PAIR_COUNT)
    }
}

// Simply copies the input frequency data for the single-symbol case
// When there's only one unique character, we don't need to select anything
fn create_initial_nodes(frequency_data: &[SymbolFrequency]) -> NodeCollection {
    frequency_data.to_vec()
}

// Checks if we have the special case of only one unique symbol
// This affects how we build the Huffman tree (single nodes get assigned "0")
fn is_single_symbol_case(frequency_data: &[SymbolFrequency]) -> bool {
    frequency_data.len() == 1
}

// Finds the nodes with the lowest frequencies for tree construction
// Sorts all nodes by frequency (with symbol as tiebreaker for consistency)
// Takes the first 'count' items from the sorted list
// Uses iterator chaining: copy data, sort it, take what we need
fn select_lowest_frequency_nodes(
    frequency_data: &[SymbolFrequency],
    count: usize,
) -> NodeCollection {
    frequency_data
        .iter()
        .copied()
        .sorted_by_frequency()
        .take(count)
        .collect()
}

trait FrequencySortable: Iterator<Item = SymbolFrequency> + Sized {
    // Sorts symbol-frequency pairs by frequency, with symbol as tiebreaker
    // Lower frequencies come first (for Huffman algorithm)
    // When frequencies are equal, sorts by symbol value for deterministic results
    // Collects into a vector first, sorts it, then returns an iterator
    fn sorted_by_frequency(self) -> std::vec::IntoIter<SymbolFrequency> {
        let mut collected: Vec<_> = self.collect();
        collected.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
        collected.into_iter()
    }
}

impl<I> FrequencySortable for I where I: Iterator<Item = SymbolFrequency> {}
