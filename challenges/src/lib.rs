#![feature(if_let_guard)]
#![expect(dead_code, reason = "WIP.")]

use std::{borrow::Borrow, cell::RefCell, marker::PhantomData, rc::Rc};

use thiserror::Error;

type Inner<T> = RefCell<Node<T>>;

#[derive(Debug)]
struct DoublyLinkedList<T> {
    start: Option<Rc<Inner<T>>>,
    end: Option<Rc<Inner<T>>>,
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

    fn init(
        new: Node<T>,
        self_start: &mut Option<Rc<Inner<T>>>,
        self_end: &mut Option<Rc<Inner<T>>>,
    ) {
        let new = Rc::new(RefCell::new(new));

        *self_end = Some(Rc::clone(&new));
        *self_start = Some(new);
    }

    pub fn insert_at(&mut self, other: T, pos: InsertionPos) {
        let new = Node {
            left: None,
            right: None,
            inner: other,
        };

        match pos {
            InsertionPos::Start => {
                let Some(start) = &self.start else {
                    return Self::init(new, &mut self.start, &mut self.end);
                };
                let mut old = start.replace(new);

                old.left = Some(Rc::clone(start));
                start.borrow_mut().right = Some(Rc::new(RefCell::new(old)));
            }
            InsertionPos::End => {
                let (Some(start), Some(end)) = (&mut self.start, &self.end) else {
                    return Self::init(new, &mut self.start, &mut self.end);
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
        }
    }

    pub fn insert_at_idx(&mut self, other: T, idx: usize) -> Result<(), InsertionError> {
        let new = Node {
            left: None,
            right: None,
            inner: other,
        };
        let Some(elem) = self.ptr_iter().nth(idx) else {
            return Err(InsertionError::IndexOutOfBounds {
                wrong_index: idx,
                actual_elements: (unsafe { self.iter() }).count(),
            });
        };
        let Some(end) = &mut self.end else {
            Self::init(new, &mut self.start, &mut self.end);
            return Ok(());
        };
        let mut old = elem.replace(new);

        old.left = Some(Rc::clone(&elem));
        old.right.clone_from(&RefCell::borrow(&elem).right);

        if let Some(ref right) = RefCell::borrow(&elem).right {
            right.borrow_mut().left = Some(Rc::new(RefCell::new(old)));
        } else {
            elem.borrow_mut().right = Some(Rc::new(RefCell::new(old)));
        }

        if Rc::ptr_eq(&elem, end) {
            *end = Rc::clone(RefCell::borrow(&elem).right.as_ref().unwrap());
        }

        Ok(())
    }

    pub fn delete<Q>(&mut self, other: &Q) -> Option<T>
    where
        T: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        let target = self
            .ptr_iter()
            .find(|elem| RefCell::borrow(elem).inner.borrow().eq(other))?;
        let (Some(start), Some(end)) = (&mut self.start, &mut self.end) else {
            return None;
        };

        // TODO: rearrange the pointers to leave only a single reference to
        //       the element about to be removed.

        eprintln!("start strong_count: {}", Rc::strong_count(start));
        eprintln!("end strong_count: {}", Rc::strong_count(end));

        let Some(target) = Rc::into_inner(target) else {
            panic!("`target` should only have a single reference at this point");
        };

        Some(target.into_inner().inner)
    }

    unsafe fn iter(&self) -> Iter<'_, T> {
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

impl<T> Default for DoublyLinkedList<T>
where
    T: PartialEq + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for DoublyLinkedList<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq + Default,
{
    fn from(value: T) -> Self {
        value.into_iter().fold(Self::new(), |mut accum, elem| {
            accum.insert_at(elem, InsertionPos::End);

            accum
        })
    }
}

#[derive(Clone, Copy)]
enum InsertionPos {
    Start,
    End,
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
struct Iter<'a, T>
where
    T: 'a,
{
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
            Some(ref mut indexer)
                if let Some(ref right) = RefCell::borrow(&Rc::clone(indexer)).right =>
            {
                *indexer = Rc::clone(right);
            }
            _ => self.current_indexer = None,
        }

        self.current_indexer.as_ref().map(Rc::clone)
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::*;

    type InsertionResult = Result<(), InsertionError>;

    #[test]
    fn insertion_at_start() -> InsertionResult {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        list.insert_at("Something else", InsertionPos::Start);
        assert_eq!(
            (unsafe { list.iter() })
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["Something else", "Something", "else"]
        );

        Ok(())
    }

    #[test]
    fn insertion_at_end() -> InsertionResult {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        list.insert_at("Nothing", InsertionPos::End);
        assert_eq!(
            (unsafe { list.iter() }
                .map(ToString::to_string)
                .collect::<Vec<_>>()),
            ["Something", "else", "Nothing",]
        );

        Ok(())
    }

    // TODO: get rid of the rest of uses of the macro and possibly make a new
    // one.

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
    fn insertion_at_idx_incorrect() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        assert!(
            list.insert_at("NUMA", InsertionPos::Index(10))
                .is_err_and(|err| {
                    matches!(
                        err,
                        InsertionError::IndexOutOfBounds {
                            wrong_index: 10,
                            actual_elements: 2,
                        }
                    )
                })
        );
    }

    #[test]
    fn search() {
        let list = DoublyLinkedList::from(["Something", "else"]);

        assert_eq!(
            (unsafe { list.iter() }).find(|elem| **elem == "else"),
            Some("else").as_ref()
        );
        assert_eq!(
            (unsafe { list.iter() }).find(|elem| **elem == "nothing"),
            None.as_ref()
        );
    }

    #[test]
    fn deletion_found() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        assert_eq!(list.delete("Something"), Some("Something"));
        assert_eq!(
            (unsafe { list.iter() })
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["else"].iter().map(ToString::to_string).collect::<Vec<_>>()
        );
    }

    #[test]
    fn deletion_not_found() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        assert_eq!(list.delete("other"), None);
        assert_eq!(
            (unsafe { list.iter() })
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["Something", "else"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        );
    }
}
