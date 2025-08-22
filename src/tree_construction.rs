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

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
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
    if frequency_map.is_empty() {
        panic!("Cannot build Huffman tree from empty frequency map");
    }

    let mut heap: BinaryHeap<HuffmanNode> = frequency_map
        .iter()
        .map(|(symbol, frequency)| HuffmanNode::new_leaf(*symbol, *frequency))
        .collect();

    if heap.len() == 1 {
        return heap.pop().expect("Heap has exactly one element");
    }

    while heap.len() > 1 {
        let node1 = heap.pop().expect("Heap has at least 2 elements");
        let node2 = heap.pop().expect("Heap has at least 1 element");

        let merged = HuffmanNode::new_internal(node1, node2);
        heap.push(merged);
    }

    heap.pop().expect("Heap should have exactly one element")
}
