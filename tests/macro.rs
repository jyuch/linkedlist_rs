use linkedlist::{list, LinkedList};

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
