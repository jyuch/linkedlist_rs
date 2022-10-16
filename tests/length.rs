use linkedlist::LinkedList;

#[test]
fn can_return_list_length_0() {
    let list: LinkedList<i32> = LinkedList::new();

    assert_eq!(0, list.len());
}

#[test]
fn can_return_list_length_1() {
    let mut list = LinkedList::new();
    list.push(1);

    assert_eq!(1, list.len());
}
