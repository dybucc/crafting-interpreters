#![expect(dead_code, reason = "WIP.")]

use std::{cell::RefCell, marker::PhantomData, ops::ControlFlow, process::id, rc::Rc};

use thiserror::Error;

#[derive(Debug)]
struct DoublyLinkedList<T> {
    start: Option<Rc<RefCell<Node<T>>>>,
    end: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Error, Debug)]
enum InsertionError {
    #[error("passed index {} out of bounds; only {} available", .wrong_index, .actual_elements)]
    IndexOutOfBounds {
        wrong_index: usize,
        actual_elements: usize,
    },
}

impl<T> DoublyLinkedList<T>
where
    T: PartialEq,
{
    fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    fn insert_at(&mut self, other: T, pos: InsertionPos) -> Result<(), InsertionError>
    where
        T: Default,
    {
        fn init<T>(
            new: Node<T>,
            self_start: &mut Option<Rc<RefCell<Node<T>>>>,
            self_end: &mut Option<Rc<RefCell<Node<T>>>>,
        ) {
            *self_start = Some(Rc::new(RefCell::new(new)));
            *self_end = Some(Rc::clone(self_start.as_ref().unwrap()));
        }

        // Exists for the purposes of performing a folding operation on the
        // container.
        #[derive(Debug)]
        enum DummyWrapper<T> {
            Some(*const T),
            None,
        }

        let new = Node {
            left: None,
            right: None,
            inner: other,
        };

        match pos {
            InsertionPos::Start => {
                let Some(start) = &self.start else {
                    init(new, &mut self.start, &mut self.end);
                    return Ok(());
                };
                let mut old = start.replace(new);

                old.left = Some(Rc::clone(start));
                start.borrow_mut().right = Some(Rc::new(RefCell::new(old)));
            }
            InsertionPos::End => {
                let (Some(start), Some(end)) = (&mut self.start, &self.end) else {
                    init(new, &mut self.start, &mut self.end);
                    return Ok(());
                };
                let mut old = end.replace(new);

                old.right = Some(Rc::clone(end));
                old.left = Some(Rc::clone(start));

                let old = Rc::new(RefCell::new(old));

                if Rc::ptr_eq(start, end) {
                    *start = old;
                    end.borrow_mut().left = Some(Rc::clone(start));
                } else {
                    end.borrow_mut().left = Some(Rc::clone(&old));
                    start.borrow_mut().right = Some(old);
                }
            }
            InsertionPos::Index(idx) => {
                let Some(_) = &self.start else {
                    init(new, &mut self.start, &mut self.end);
                    return Ok(());
                };

                let dummy = DummyWrapper::None;
                let Some(DummyWrapper::Some(elem)) = self
                    .iter()
                    .enumerate()
                    .try_fold(dummy, |_, (actual_idx, elem)| {
                        (actual_idx == idx).then_some(DummyWrapper::Some(elem))
                    })
                else {
                    return Err(InsertionError::IndexOutOfBounds {
                        wrong_index: idx,
                        actual_elements: self.iter().count(),
                    });
                };

                todo!("Handle cases where the index is correct.");
            }
        }

        Ok(())
    }

    fn delete(&mut self, other: &T) -> T {
        todo!()
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            first: self.start.as_ref().map(|elem| elem.as_ptr().cast_const()),
            last: self.end.as_ref().map(|elem| elem.as_ptr().cast_const()),
            current: None,
            _marker: PhantomData,
        }
    }
}

impl<U> From<U> for DoublyLinkedList<U::Item>
where
    U: IntoIterator,
    U::Item: PartialEq + Default,
{
    fn from(value: U) -> Self {
        value.into_iter().fold(
            Self {
                start: None,
                end: None,
            },
            |mut accum, elem| {
                accum.insert_at(elem, InsertionPos::End).expect(
                    "conversion shouldn't fail because only index-based insertion is prone to \
                    failure",
                );

                accum
            },
        )
    }
}

#[derive(Clone, Copy)]
enum InsertionPos {
    Start,
    End,
    Index(usize),
}

#[derive(Debug)]
struct Node<T> {
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    inner: T,
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

#[derive(Debug)]
struct Iter<'a, T> {
    first: Option<*const Node<T>>,
    last: Option<*const Node<T>>,
    current: Option<*const Node<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => {
                let Some(first) = &self.first else {
                    return None;
                };

                self.current = Some(&raw const **first);
            }
            Some(item) => {
                let Some(next) = &(unsafe { &*item }).right else {
                    return None;
                };

                self.current = Some(next.as_ptr().cast_const());
            }
        }

        self.current.map(|elem| &(unsafe { &*elem }).inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! insertion_test {
        ($src:expr, $insertion:expr, $list:expr, $pos:expr) => {{
            $src.insert_at($insertion, $pos);

            assert_eq!(
                $src.iter().map(|elem| elem.to_string()).collect::<Vec<_>>(),
                $list
                    .iter()
                    .map(|elem| elem.to_string())
                    .collect::<Vec<_>>()
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

        assert_eq!(
            list.iter().find(|elem| **elem == "else"),
            Some("else").as_ref()
        );
        assert_eq!(list.iter().find(|elem| **elem == "nothing"), None.as_ref());
    }

    #[test]
    fn deletion() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        assert_eq!(list.delete(&"Something"), "Something");
    }
}
