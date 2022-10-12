use linkedlist::LinkedList;

#[test]
fn can_push_and_pop_i32_items() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(Some(3), list.pop());
    assert_eq!(Some(2), list.pop());
    assert_eq!(Some(1), list.pop());
    assert_eq!(None, list.pop());
}

#[test]
fn can_push_and_pop_string_items() {
    let mut list = LinkedList::new();

    list.push("1");
    list.push("2");
    list.push("3");

    assert_eq!(Some("3"), list.pop());
    assert_eq!(Some("2"), list.pop());
    assert_eq!(Some("1"), list.pop());
    assert_eq!(None, list.pop());
}

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
