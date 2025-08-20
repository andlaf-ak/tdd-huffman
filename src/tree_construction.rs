use crate::node_selection::SymbolFrequency;

#[derive(Debug, PartialEq, Clone)]
pub enum HuffmanChild {
    Leaf(SymbolFrequency),
    Node(Box<HuffmanNode>),
}

impl HuffmanChild {
    fn frequency(&self) -> usize {
        match self {
            HuffmanChild::Leaf((_, freq)) => *freq,
            HuffmanChild::Node(node) => node.frequency,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HuffmanNode {
    frequency: usize,
    symbol: Option<u8>,
    left_child: Option<HuffmanChild>,
    right_child: Option<HuffmanChild>,
}

impl HuffmanNode {
    fn new(left_child: HuffmanChild, right_child: HuffmanChild) -> Self {
        let frequency = left_child.frequency() + right_child.frequency();
        Self {
            frequency,
            symbol: None,
            left_child: Some(left_child),
            right_child: Some(right_child),
        }
    }

    fn new_internal(left: SymbolFrequency, right: SymbolFrequency) -> Self {
        Self::new(HuffmanChild::Leaf(left), HuffmanChild::Leaf(right))
    }

    fn new_mixed(left_node: HuffmanNode, right_leaf: SymbolFrequency) -> Self {
        Self::new(
            HuffmanChild::Node(Box::new(left_node)),
            HuffmanChild::Leaf(right_leaf),
        )
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }

    pub fn symbol(&self) -> Option<u8> {
        self.symbol
    }

    pub fn left_child(&self) -> Option<&SymbolFrequency> {
        match &self.left_child {
            Some(HuffmanChild::Leaf(leaf)) => Some(leaf),
            _ => None,
        }
    }

    pub fn right_child(&self) -> Option<&SymbolFrequency> {
        match &self.right_child {
            Some(HuffmanChild::Leaf(leaf)) => Some(leaf),
            _ => None,
        }
    }

    pub fn left_child_node(&self) -> Option<&HuffmanNode> {
        match &self.left_child {
            Some(HuffmanChild::Node(node)) => Some(node),
            _ => None,
        }
    }

    pub fn right_child_leaf(&self) -> Option<&SymbolFrequency> {
        match &self.right_child {
            Some(HuffmanChild::Leaf(leaf)) => Some(leaf),
            _ => None,
        }
    }
}

pub fn merge_leaf_nodes(left: SymbolFrequency, right: SymbolFrequency) -> HuffmanNode {
    HuffmanNode::new_internal(left, right)
}

pub fn merge_with_leaf_node(internal_node: HuffmanNode, leaf: SymbolFrequency) -> HuffmanNode {
    HuffmanNode::new_mixed(internal_node, leaf)
}
