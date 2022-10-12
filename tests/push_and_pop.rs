use linkedlist::LinkedList;

#[test]
fn can_push_and_pop_items() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(Some(3), list.pop());
    assert_eq!(Some(2), list.pop());
    assert_eq!(Some(1), list.pop());
}
