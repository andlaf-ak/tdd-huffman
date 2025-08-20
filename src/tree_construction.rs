use crate::node_selection::SymbolFrequency;

#[derive(Debug, PartialEq, Clone)]
pub struct HuffmanNode {
    frequency: usize,
    symbol: Option<u8>,
    left_child: Option<Box<HuffmanNode>>,
    right_child: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new_leaf(symbol: u8, frequency: usize) -> Self {
        Self {
            frequency,
            symbol: Some(symbol),
            left_child: None,
            right_child: None,
        }
    }

    fn new_internal(left_child: HuffmanNode, right_child: HuffmanNode) -> Self {
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

    pub fn left_child_node(&self) -> Option<&HuffmanNode> {
        self.left_child.as_ref().map(|boxed| boxed.as_ref())
    }

    pub fn right_child_node(&self) -> Option<&HuffmanNode> {
        self.right_child.as_ref().map(|boxed| boxed.as_ref())
    }

    // Better API methods
    pub fn as_leaf(&self) -> Option<(u8, usize)> {
        self.symbol.map(|s| (s, self.frequency))
    }
}

pub fn merge_leaf_nodes(left: SymbolFrequency, right: SymbolFrequency) -> HuffmanNode {
    let left_node = HuffmanNode::new_leaf(left.0, left.1);
    let right_node = HuffmanNode::new_leaf(right.0, right.1);
    HuffmanNode::new_internal(left_node, right_node)
}

pub fn merge_with_leaf_node(internal_node: HuffmanNode, leaf: SymbolFrequency) -> HuffmanNode {
    let leaf_node = HuffmanNode::new_leaf(leaf.0, leaf.1);
    HuffmanNode::new_internal(internal_node, leaf_node)
}

pub fn merge_internal_nodes(left_node: HuffmanNode, right_node: HuffmanNode) -> HuffmanNode {
    HuffmanNode::new_internal(left_node, right_node)
}
