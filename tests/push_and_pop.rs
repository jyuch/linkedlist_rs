use linkedlist::{list, LinkedList};

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

#[test]
fn can_use_list_macro_with_no_expr() {
    let expect: LinkedList<i32> = LinkedList::new();
    let actual: LinkedList<i32> = list![];
    assert!(expect.into_iter().eq(actual.into_iter()));
}

#[test]
fn can_use_list_macro_with_many_expr() {
    let mut expect: LinkedList<i32> = LinkedList::new();
    expect.push(1);
    expect.push(2);
    expect.push(3);

    let actual: LinkedList<i32> = list![1, 2, 3];
    assert!(expect.into_iter().eq(actual.into_iter()));
}
