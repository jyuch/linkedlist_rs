use crate::Cell::{Cons, Nil};
use std::mem::replace;

#[derive(Debug)]
enum Cell {
    Nil,
    Cons(i32, Box<Cell>),
}

pub struct LinkedList {
    head: Cell,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: Nil }
    }

    pub fn push(&mut self, value: i32) {
        let next = Cons(value, Box::new(replace(&mut self.head, Nil)));
        self.head = next;
    }

    pub fn pop(&mut self) -> Option<i32> {
        match replace(&mut self.head, Nil) {
            Nil => None,
            Cons(v, n) => {
                self.head = *n;
                Some(v)
            }
        }
    }
}
