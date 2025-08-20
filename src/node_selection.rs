pub type SymbolFrequency = (u8, usize);
pub type NodeCollection = Vec<SymbolFrequency>;

/// Selects nodes for Huffman tree construction.
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

fn create_initial_nodes(frequency_data: &[SymbolFrequency]) -> NodeCollection {
    frequency_data.to_vec()
}

fn is_single_symbol_case(frequency_data: &[SymbolFrequency]) -> bool {
    frequency_data.len() == 1
}

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
    fn sorted_by_frequency(self) -> std::vec::IntoIter<SymbolFrequency> {
        let mut collected: Vec<_> = self.collect();
        collected.sort_by(|a, b| {
            a.1.cmp(&b.1) // Primary: frequency comparison
                .then_with(|| a.0.cmp(&b.0)) // Secondary: symbol comparison for tie-breaking
        });
        collected.into_iter()
    }
}

impl<I> FrequencySortable for I where I: Iterator<Item = SymbolFrequency> {}
