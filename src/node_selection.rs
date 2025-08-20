/// Type alias for symbol-frequency pairs used in Huffman tree construction.
pub type SymbolFrequency = (u8, usize);

/// Type alias for collections of nodes in Huffman tree construction.
pub type NodeCollection = Vec<SymbolFrequency>;

/// Selects nodes from frequency data for Huffman tree construction.
///
/// This function processes symbol-frequency pairs and selects appropriate nodes
/// for building a Huffman tree. The selection strategy depends on the input size:
/// - Single symbol: Returns all input (the single node)  
/// - Multiple symbols: Returns the 2 nodes with lowest frequencies
///
/// # Examples
///
/// ```
/// use tdd_huffman::select_nodes;
///
/// // Single symbol case
/// let input = [(65u8, 1usize)]; // 'A' with frequency 1
/// let nodes = select_nodes(&input);
/// assert_eq!(nodes.len(), 1);
/// assert_eq!(nodes[0], (65u8, 1usize));
///
/// // Multiple symbols case - selects 2 lowest frequencies
/// let input = [(65u8, 5usize), (66u8, 2usize), (67u8, 8usize), (68u8, 3usize)];
/// let nodes = select_nodes(&input);
/// assert_eq!(nodes.len(), 2);
/// assert!(nodes.contains(&(66u8, 2usize))); // B:2 (lowest)
/// assert!(nodes.contains(&(68u8, 3usize))); // D:3 (second lowest)
/// ```
pub fn select_nodes(frequency_data: &[SymbolFrequency]) -> NodeCollection {
    // Constants for readability
    const HUFFMAN_PAIR_COUNT: usize = 2;

    // Handle different cases based on input size
    if is_single_symbol_case(frequency_data) {
        // Single symbol case: return all nodes
        create_initial_nodes(frequency_data)
    } else {
        // Multiple symbols case: select the nodes with lowest frequencies for Huffman pairing
        select_lowest_frequency_nodes(frequency_data, HUFFMAN_PAIR_COUNT)
    }
}

/// Creates initial nodes from frequency data.
///
/// Each symbol-frequency pair becomes a potential node for the Huffman tree.
fn create_initial_nodes(frequency_data: &[SymbolFrequency]) -> NodeCollection {
    frequency_data.to_vec()
}

/// Determines if this is a single symbol case.
///
/// Single symbol cases require special handling in Huffman compression.
fn is_single_symbol_case(frequency_data: &[SymbolFrequency]) -> bool {
    frequency_data.len() <= 1
}

/// Selects the specified number of nodes with the lowest frequencies.
///
/// Sorts the input by frequency and returns the lowest `count` nodes.
/// This is used for Huffman tree construction where nodes with minimum
/// frequencies are paired together first.
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

/// Extension trait to add frequency-based sorting to iterators.
trait FrequencySortable: Iterator<Item = SymbolFrequency> + Sized {
    /// Sorts symbol-frequency pairs by frequency in ascending order.
    fn sorted_by_frequency(self) -> std::vec::IntoIter<SymbolFrequency> {
        let mut collected: Vec<_> = self.collect();
        collected.sort_by(|a, b| a.1.cmp(&b.1));
        collected.into_iter()
    }
}

impl<I> FrequencySortable for I where I: Iterator<Item = SymbolFrequency> {}
