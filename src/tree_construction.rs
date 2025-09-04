use crate::frequency_map::ByteFrequencyMap;
use crate::node_selection::SymbolFrequency;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug)]
pub struct HuffmanNode {
    frequency: usize,
    symbol: Option<u8>,
    left_child: Option<Box<HuffmanNode>>,
    right_child: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    // Creates a leaf node representing a single character and its frequency
    pub fn new_leaf(symbol: u8, frequency: usize) -> Self {
        Self {
            frequency,
            symbol: Some(symbol),
            left_child: None,
            right_child: None,
        }
    }

    // Creates an internal node by combining two child nodes
    // The frequency becomes the sum of both children's frequencies
    pub fn new_internal(left_child: HuffmanNode, right_child: HuffmanNode) -> Self {
        Self {
            frequency: left_child.frequency + right_child.frequency,
            symbol: None,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        }
    }

    // Returns how often this node's symbol(s) appear in the input
    pub fn frequency(&self) -> usize {
        self.frequency
    }

    // Returns the byte value for leaf nodes, None for internal nodes
    pub fn symbol(&self) -> Option<u8> {
        self.symbol
    }

    // Returns true if this is a leaf node (contains a symbol)
    pub fn is_leaf(&self) -> bool {
        self.symbol.is_some()
    }

    // Returns the left child node for internal nodes, None for leaf nodes
    pub fn left_child(&self) -> Option<&HuffmanNode> {
        self.left_child.as_deref()
    }

    // Returns the right child node for internal nodes, None for leaf nodes
    pub fn right_child(&self) -> Option<&HuffmanNode> {
        self.right_child.as_deref()
    }

    // Returns (symbol, frequency) tuple if this is a leaf node
    pub fn as_leaf(&self) -> Option<(u8, usize)> {
        self.symbol.map(|s| (s, self.frequency))
    }
}

impl Ord for HuffmanNode {
    // Compares nodes by frequency for priority queue ordering
    // Lower frequencies have higher priority (reversed comparison)
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanNode {
    // Partial comparison implementation required for Ord trait
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Creates an internal node from two symbol-frequency pairs
// Converts each pair into a leaf node, then combines them
pub fn merge_leaf_nodes(left: SymbolFrequency, right: SymbolFrequency) -> HuffmanNode {
    let left_node = HuffmanNode::new_leaf(left.0, left.1);
    let right_node = HuffmanNode::new_leaf(right.0, right.1);
    HuffmanNode::new_internal(left_node, right_node)
}

// Combines two existing Huffman nodes into a single internal node
pub fn merge_nodes(left: HuffmanNode, right: HuffmanNode) -> HuffmanNode {
    HuffmanNode::new_internal(left, right)
}

// Builds the final Huffman tree from a priority queue of nodes
// Repeatedly removes the two lowest-frequency nodes and merges them
// Continues until only one node remains (the root of the tree)
// Uses an iterator pattern instead of a traditional while loop
fn build_tree_from_heap(mut heap: BinaryHeap<HuffmanNode>) -> HuffmanNode {
    if heap.len() == 1 {
        return heap.pop().expect("Heap has exactly one element");
    }

    std::iter::from_fn(|| {
        if heap.len() > 1 {
            let node1 = heap.pop().expect("Heap has at least 2 elements");
            let node2 = heap.pop().expect("Heap has at least 1 element");
            let merged = HuffmanNode::new_internal(node1, node2);
            heap.push(merged);
            Some(())
        } else {
            None
        }
    })
    .last();

    heap.pop().expect("Heap should have exactly one element")
}

// Builds a Huffman tree from byte frequency data
// Step 1: Convert each (byte, frequency) pair into a leaf node
// Step 2: Put all leaf nodes into a priority queue (heap)
// Step 3: Repeatedly merge the two lowest-frequency nodes until one remains
// The resulting tree assigns shorter codes to more frequent bytes
pub fn build_huffman_tree(frequency_map: &ByteFrequencyMap) -> HuffmanNode {
    if frequency_map.is_empty() {
        panic!("Cannot build Huffman tree from empty frequency map");
    }

    let heap: BinaryHeap<HuffmanNode> = frequency_map
        .iter()
        .map(|(symbol, frequency)| HuffmanNode::new_leaf(*symbol, *frequency))
        .collect();

    build_tree_from_heap(heap)
}
