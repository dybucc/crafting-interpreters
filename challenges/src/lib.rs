#![expect(dead_code, reason = "WIP.")]

use std::{marker::PhantomData, mem};

struct DoublyLinkedList<T> {
    start: Option<Box<Node<T>>>,
    end: Option<Box<Node<T>>>,
    _marker: PhantomData<T>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            start: None,
            end: None,
            _marker: PhantomData,
        }
    }

    fn insert_at(&mut self, other: T, pos: InsertionPos) -> Option<()> {
        match pos {
            InsertionPos::Start => {
                let new = Node {
                    left: None,
                    right: None,
                    inner: other,
                };
                let Some(start) = &mut self.start else {
                    self.start = Some(Box::new(new));
                    return Some(());
                };

                let mut new = Box::new(new);
                mem::swap(&mut new, start);
            }
            InsertionPos::End => todo!(),
            InsertionPos::Index(idx) => todo!(),
        }

        Some(())
    }

    fn delete(&mut self, other: &T) -> T {
        todo!()
    }

    fn find(&self, other: &T) -> Option<usize> {
        todo!()
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            first: self.start.as_deref(),
            last: self.end.as_deref(),
            current: None,
            _marker: PhantomData,
        }
    }
}

impl<U> From<U> for DoublyLinkedList<U::Item>
where
    U: IntoIterator,
{
    fn from(value: U) -> Self {
        value.into_iter().fold(
            Self {
                start: None,
                end: None,
                _marker: PhantomData,
            },
            |mut accum, elem| {
                accum.insert_at(elem, InsertionPos::Start);

                accum
            },
        )
    }
}

enum InsertionPos {
    Start,
    End,
    Index(usize),
}

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    inner: T,
}

struct Iter<'a, T> {
    first: Option<&'a Node<T>>,
    last: Option<&'a Node<T>>,
    current: Option<&'a Node<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        macro_rules! insertion_test {
            ($src:expr, $insertion:expr, $list:expr, $pos:expr) => {{
                $src.insert_at($insertion, $pos);
                assert!(
                    $src.iter()
                        .map(|elem| elem.as_bytes())
                        .eq($list.iter().map(|elem| elem.as_bytes()))
                );
            }};
        }

        let mut list = DoublyLinkedList::from(["Something", "else"]);

        insertion_test!(
            list,
            "Something else",
            ["Something else", "Something", "else"],
            InsertionPos::Start
        );
        insertion_test!(
            list,
            "Nothing",
            ["Something else", "Something", "else", "Nothing",],
            InsertionPos::End
        );
        insertion_test!(
            list,
            "NUMA",
            ["Something else", "Something", "NUMA", "else", "Nothing",],
            InsertionPos::Index(2)
        );

        assert_eq!(list.find(&"else"), Some(1));
        assert_eq!(list.find(&"nothing"), None);

        assert_eq!(list.delete(&"Something"), "Something");
    }
}
