#![expect(dead_code, reason = "WIP.")]

use std::{
    cell::{Ref, RefCell},
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
};

struct DoublyLinkedList<T> {
    start: Option<Rc<RefCell<Node<T>>>>,
    end: Option<Rc<RefCell<Node<T>>>>,
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
        let new = Node {
            left: None,
            right: None,
            inner: other,
        };

        match pos {
            InsertionPos::Start => {
                let Some(start) = &self.start else {
                    self.start = Some(Rc::new(RefCell::new(new)));
                    self.end = Some(Rc::clone(self.start.as_ref().unwrap()));

                    return Some(());
                };
                let mut old = start.replace(new);

                old.left = Some(Rc::clone(start));
                start.borrow_mut().right = Some(Rc::new(RefCell::new(old)));
            }
            InsertionPos::End => {
                let Some(end) = &self.end else {
                    self.start = Some(Rc::new(RefCell::new(new)));
                    self.end = Some(Rc::clone(self.start.as_ref().unwrap()));

                    return Some(());
                };
                let mut old = end.replace(new);

                old.right = Some(Rc::clone(end));
                end.borrow_mut().left = Some(Rc::new(RefCell::new(old)));
            }
            InsertionPos::Index(idx) => (),
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
            first: self.start.as_ref().map(|elem| elem.as_ref().borrow()),
            last: self.end.as_ref().map(|elem| elem.as_ref().borrow()),
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
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    inner: T,
}

struct Iter<'a, T> {
    first: Option<Ref<'a, Node<T>>>,
    last: Option<Ref<'a, Node<T>>>,
    current: Option<Rc<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => {
                let Some(first) = &self.first else {
                    return None;
                };
                let Some(last) = &self.last else {
                    return None;
                };
            }
            Some(_) => todo!(),
        }

        self.first.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn insertion_at_start() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        insertion_test!(
            list,
            "Something else",
            ["Something else", "Something", "else"],
            InsertionPos::Start
        );
    }

    #[test]
    fn insertion_at_end() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        insertion_test!(
            list,
            "Nothing",
            ["Something", "else", "Nothing",],
            InsertionPos::End
        );
    }

    #[test]
    fn insertion_at_idx_correct() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        insertion_test!(
            list,
            "NUMA",
            ["Something", "NUMA", "else"],
            InsertionPos::Index(1)
        );
    }

    #[test]
    fn insertion_at_idx_incorrect() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        todo!();
    }

    #[test]
    fn search() {
        let list = DoublyLinkedList::from(["Something", "else"]);

        assert_eq!(list.find(&"else"), Some(1));
        assert_eq!(list.find(&"nothing"), None);
    }

    #[test]
    fn deletion() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        assert_eq!(list.delete(&"Something"), "Something");
    }
}
