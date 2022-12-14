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

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut head = &self.head;

        while let Cons(_, n) = head {
            length += 1;
            head = &n;
        }

        length
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { current: self.head }
    }
}

pub struct IntoIter<T> {
    current: Cell<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match replace(&mut self.current, Nil) {
            Nil => None,
            Cons(v, n) => {
                self.current = *n;
                Some(v)
            }
        }
    }
}

pub struct IntoIterRef<'a, T> {
    current: &'a Cell<T>,
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = IntoIterRef<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterRef {
            current: &self.head,
        }
    }
}

impl<'a, T> Iterator for IntoIterRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match replace(&mut self.current, &Nil) {
            Nil => None,
            Cons(v, n) => {
                self.current = n;
                Some(v)
            }
        }
    }
}

#[macro_export]
macro_rules! list {
    () => {
        $crate::LinkedList::new()
    };
    ( $( $x:expr ),* ) => {
        {
            let mut temp  = $crate::LinkedList::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}
