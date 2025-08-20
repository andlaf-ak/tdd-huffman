use crate::node_selection::SymbolFrequency;

#[derive(Debug, PartialEq, Clone)]
pub struct HuffmanNode {
    frequency: usize,
    symbol: Option<u8>,
    left_child: Option<SymbolFrequency>,
    right_child: Option<SymbolFrequency>,
}

impl HuffmanNode {
    fn new_internal(left: SymbolFrequency, right: SymbolFrequency) -> Self {
        Self {
            frequency: left.1 + right.1,
            symbol: None,
            left_child: Some(left),
            right_child: Some(right),
        }
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }

    pub fn symbol(&self) -> Option<u8> {
        self.symbol
    }

    pub fn left_child(&self) -> Option<&SymbolFrequency> {
        self.left_child.as_ref()
    }

    pub fn right_child(&self) -> Option<&SymbolFrequency> {
        self.right_child.as_ref()
    }
}

pub fn merge_leaf_nodes(left: SymbolFrequency, right: SymbolFrequency) -> HuffmanNode {
    HuffmanNode::new_internal(left, right)
}
