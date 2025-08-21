use crate::frequency_map::ByteFrequencyMap;
use crate::node_selection::SymbolFrequency;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
pub struct HuffmanNode {
    frequency: usize,
    symbol: Option<u8>,
    left_child: Option<Box<HuffmanNode>>,
    right_child: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    pub fn new_leaf(symbol: u8, frequency: usize) -> Self {
        Self {
            frequency,
            symbol: Some(symbol),
            left_child: None,
            right_child: None,
        }
    }

    pub fn new_internal(left_child: HuffmanNode, right_child: HuffmanNode) -> Self {
        Self {
            frequency: left_child.frequency + right_child.frequency,
            symbol: None,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        }
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }

    pub fn symbol(&self) -> Option<u8> {
        self.symbol
    }

    pub fn is_leaf(&self) -> bool {
        self.symbol.is_some()
    }

    pub fn left_child(&self) -> Option<&HuffmanNode> {
        self.left_child.as_deref()
    }

    pub fn right_child(&self) -> Option<&HuffmanNode> {
        self.right_child.as_deref()
    }

    pub fn as_leaf(&self) -> Option<(u8, usize)> {
        self.symbol.map(|s| (s, self.frequency))
    }
}

// Implement ordering for HuffmanNode to work with BinaryHeap
// BinaryHeap is a max-heap, but we want min-heap behavior (lowest frequency first)
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the comparison to get min-heap behavior
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn merge_leaf_nodes(left: SymbolFrequency, right: SymbolFrequency) -> HuffmanNode {
    let left_node = HuffmanNode::new_leaf(left.0, left.1);
    let right_node = HuffmanNode::new_leaf(right.0, right.1);
    HuffmanNode::new_internal(left_node, right_node)
}

pub fn merge_nodes(left: HuffmanNode, right: HuffmanNode) -> HuffmanNode {
    HuffmanNode::new_internal(left, right)
}

pub fn build_huffman_tree(frequency_map: &ByteFrequencyMap) -> HuffmanNode {
    match frequency_map.len() {
        0 => panic!("Cannot build Huffman tree from empty frequency map"),
        1 => {
            let (symbol, frequency) = frequency_map
                .iter()
                .next()
                .expect("Map has exactly one element");
            HuffmanNode::new_leaf(*symbol, *frequency)
        }
        2 => {
            // Handle two symbols case - create two leaf nodes and merge them
            let mut nodes = frequency_map
                .iter()
                .map(|(symbol, freq)| HuffmanNode::new_leaf(*symbol, *freq));

            let leaf1 = nodes.next().expect("First element exists");
            let leaf2 = nodes.next().expect("Second element exists");

            HuffmanNode::new_internal(leaf1, leaf2)
        }
        _ => {
            // Handle 3+ symbols using priority queue (min-heap)
            let mut heap = BinaryHeap::new();

            // Convert frequency map to priority queue of leaf nodes
            for (symbol, frequency) in frequency_map.iter() {
                heap.push(HuffmanNode::new_leaf(*symbol, *frequency));
            }

            // Repeatedly merge two lowest-frequency nodes until only one remains
            while heap.len() > 1 {
                let node1 = heap.pop().expect("Heap has at least 2 elements");
                let node2 = heap.pop().expect("Heap has at least 1 element");

                // Create internal node and push back to heap
                let merged = HuffmanNode::new_internal(node1, node2);
                heap.push(merged);
            }

            // The remaining node is the root of the Huffman tree
            heap.pop().expect("Heap should have exactly one element")
        }
    }
}
