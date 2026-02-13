#![feature(if_let_guard)]
#![expect(dead_code, reason = "WIP.")]

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

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
    T: PartialEq + Default,
{
    fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    fn insert_at(&mut self, other: T, pos: InsertionPos) -> Result<(), InsertionError> {
        fn init<T>(
            new: Node<T>,
            self_start: &mut Option<Rc<RefCell<Node<T>>>>,
            self_end: &mut Option<Rc<RefCell<Node<T>>>>,
        ) {
            let new = Rc::new(RefCell::new(new));

            *self_end = Some(Rc::clone(&new));
            *self_start = Some(new);
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
                let Some(elem) = self.ptr_iter().nth(idx) else {
                    return Err(InsertionError::IndexOutOfBounds {
                        wrong_index: idx,
                        actual_elements: self.iter().count(),
                    });
                };
                let Some(end) = &mut self.end else {
                    init(new, &mut self.start, &mut self.end);
                    return Ok(());
                };
                let mut old = elem.replace(new);

                old.left = Some(Rc::clone(&elem));
                old.right.clone_from(&elem.borrow().right);

                if let Some(ref right) = elem.borrow().right {
                    right.borrow_mut().left = Some(Rc::new(RefCell::new(old)));
                } else {
                    elem.borrow_mut().right = Some(Rc::new(RefCell::new(old)));
                }

                if Rc::ptr_eq(&elem, end) {
                    *end = Rc::clone(elem.borrow().right.as_ref().unwrap());
                }
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

    fn ptr_iter(&self) -> PtrIter<T> {
        PtrIter {
            start: self.start.clone(),
            end: self.end.clone(),
            current: 0,
            current_indexer: None,
        }
    }
}

impl<T> From<T> for DoublyLinkedList<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq + Default,
{
    fn from(value: T) -> Self {
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

struct PtrIter<T> {
    start: Option<Rc<RefCell<Node<T>>>>,
    end: Option<Rc<RefCell<Node<T>>>>,
    current: usize,
    current_indexer: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for PtrIter<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(start) = &self.start else {
            return None;
        };

        match self.current_indexer {
            None => self.current_indexer = Some(Rc::clone(start)),
            Some(ref mut indexer) if let Some(ref right) = Rc::clone(indexer).borrow().right => {
                *indexer = Rc::clone(right);
            }
            _ => self.current_indexer = None,
        }

        self.current_indexer.as_ref().map(Rc::clone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! insertion_test {
        ($src:expr, $insertion:expr, $list:expr, $pos:expr$(,)?) => {{
            $src.insert_at($insertion, $pos)?;

            assert!(
                $src.iter()
                    .map(|elem| elem.to_string())
                    .eq($list.iter().map(|elem| elem.to_string())),
            );
        }};
    }

    type InsertionResult = Result<(), InsertionError>;

    #[test]
    fn insertion_at_start() -> InsertionResult {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        insertion_test!(
            list,
            "Something else",
            ["Something else", "Something", "else"],
            InsertionPos::Start,
        );

        Ok(())
    }

    #[test]
    fn insertion_at_end() -> InsertionResult {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        insertion_test!(
            list,
            "Nothing",
            ["Something", "else", "Nothing",],
            InsertionPos::End,
        );

        Ok(())
    }

    #[test]
    fn insertion_at_idx_correct() -> InsertionResult {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        insertion_test!(
            list,
            "NUMA",
            ["Something", "NUMA", "else"],
            InsertionPos::Index(1),
        );

        Ok(())
    }

    #[test]
    fn insertion_at_idx_incorrect() -> InsertionResult {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        assert!(
            list.insert_at("NUMA", InsertionPos::Index(10))
                .is_err_and(|err| {
                    if let InsertionError::IndexOutOfBounds {
                        wrong_index,
                        actual_elements,
                    } = err
                    {
                        wrong_index == 10 && actual_elements == 2
                    } else {
                        false
                    }
                })
        );

        Ok(())
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
