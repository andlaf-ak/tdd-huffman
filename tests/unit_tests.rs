use tdd_huffman::add;

#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
