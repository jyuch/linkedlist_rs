use crate::Cell::{Cons, Nil};
use std::mem::replace;

#[derive(Debug)]
enum Cell<T> {
    Nil,
    Cons(T, Box<Cell<T>>),
}

pub struct LinkedList<T> {
    head: Cell<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: Nil }
    }

    pub fn push(&mut self, value: T) {
        let next = Cons(value, Box::new(replace(&mut self.head, Nil)));
        self.head = next;
    }

    pub fn pop(&mut self) -> Option<T> {
        match replace(&mut self.head, Nil) {
            Nil => None,
            Cons(v, n) => {
                self.head = *n;
                Some(v)
            }
        }
    }
}
