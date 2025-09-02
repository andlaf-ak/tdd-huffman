use tdd_huffman::{serialize_tree, serialize_tree_to_bits, HuffmanNode, OutputBitStream};

fn main() {
    // Test case that's failing: tree with two leaves
    let left_leaf = HuffmanNode::new_leaf(65u8, 3);  // 'A' = 01000001
    let right_leaf = HuffmanNode::new_leaf(66u8, 5); // 'B' = 01000010
    let tree = HuffmanNode::new_internal(left_leaf, right_leaf);

    let string_result = serialize_tree(&tree);
    println!("String result: {}", string_result);
    println!("Expected:      0101000001101000010");
    
    // Let's see what bits are produced
    let mut output = Vec::new();
    let mut bit_stream = OutputBitStream::new(&mut output);
    serialize_tree_to_bits(&tree, &mut bit_stream).unwrap();
    bit_stream.flush().unwrap();
    
    println!("Raw bytes: {:?}", output);
    
    // Convert bytes to bits manually
    for (i, &byte) in output.iter().enumerate() {
        print!("Byte {}: {:08b} = ", i, byte);
        for bit_pos in (0..8).rev() {
            let bit = (byte >> bit_pos) & 1;
            print!("{}", bit);
        }
        println!();
    }
    
    // Expected structure:
    // 0 (internal node)
    // 1 01000001 (leaf A) 
    // 1 01000010 (leaf B)
    // = 0 + 1 + 01000001 + 1 + 01000010 = 0101000001101000010 (19 bits)
    
    println!("\nExpected bit sequence:");
    println!("0 - internal node");
    println!("1 01000001 - leaf A (65)");  
    println!("1 01000010 - leaf B (66)");
    println!("Total: 0101000001101000010 (19 bits)");
    
    // 19 bits = 2 bytes + 3 bits
    // First byte: 01010000 = 80
    // Second byte: 01101000 = 104  
    // Third byte: 010????? = 64 (with padding)
    
    println!("\nExpected bytes:");
    println!("Byte 0: 01010000 = {}", 0b01010000);
    println!("Byte 1: 01101000 = {}", 0b01101000);
    println!("Byte 2: 01000000 = {} (3 bits + 5 padding)", 0b01000000);
}
