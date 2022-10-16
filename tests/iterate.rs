use linkedlist::LinkedList;

#[test]
fn list_can_iterate() {
    let mut list = LinkedList::new();
    list.push(1);
    let mut iter = list.into_iter();

    assert_eq!(Some(1), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn list_can_iterate_for() {
    let mut list = LinkedList::new();
    list.push(1);

    for i in &list {
        assert_eq!(1, *i);
    }

    for i in &list {
        assert_eq!(1, *i);
    }
}
