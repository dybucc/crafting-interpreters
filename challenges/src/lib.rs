#![expect(dead_code, reason = "WIP.")]

use std::{borrow::Borrow, cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

use thiserror::Error;

type Inner<T> = RefCell<Node<T>>;

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    start: Option<Rc<Inner<T>>>,
    end: Option<Rc<Inner<T>>>,
}

#[derive(Error, Debug)]
pub enum InsertionError {
    #[error("passed index {} out of bounds; only {} available", .wrong_index, .actual_elements)]
    IndexOutOfBounds {
        wrong_index: usize,
        actual_elements: usize,
    },
}

impl<T: PartialEq + Default> DoublyLinkedList<T> {
    #[expect(
        clippy::must_use_candidate,
        reason = "It's not a bug for a list to be discarded."
    )]
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    fn init_empty(&mut self, new: Node<T>) {
        let new = Rc::new(RefCell::new(new));

        self.end = Some(Rc::clone(&new));
        self.start = Some(new);
    }

    pub fn insert_at(&mut self, other: T, pos: InsertionPos)
    where
        T: Debug,
    {
        let new = Node {
            left: None,
            right: None,
            inner: other,
        };

        // TODO: finish fixing up insertion op as the tests for start and end
        // insertion don't seem to work.

        eprintln!("state of the list: {:?}", self.iter().collect::<Vec<_>>());

        match pos {
            InsertionPos::Start => {
                let (Some(start), Some(end)) = (&self.start, &mut self.end) else {
                    return self.init_empty(new);
                };
                let mut old = start.replace(new);

                old.left = Some(Rc::clone(start));
                old.right.clone_from(&RefCell::borrow(start).right);

                let old = Rc::new(RefCell::new(old));

                if Rc::ptr_eq(start, end) {
                    *end = old;
                    start.borrow_mut().right = Some(Rc::clone(end));
                } else {
                    start
                        .borrow_mut()
                        .right
                        .as_ref()
                        .expect(
                            "if `start` is not equivalent to `end` then surely there's something \
                            to the right of `start`",
                        )
                        .borrow_mut()
                        .left = Some(Rc::clone(&old));
                    start.borrow_mut().right = Some(old);
                }
            }
            InsertionPos::End => {
                let (Some(start), Some(end)) = (&mut self.start, &self.end) else {
                    return self.init_empty(new);
                };
                let mut old = end.replace(new);

                old.right = Some(Rc::clone(end));
                old.left.clone_from(&RefCell::borrow(end).left);

                let old = Rc::new(RefCell::new(old));

                if Rc::ptr_eq(start, end) {
                    *start = old;
                    end.borrow_mut().left = Some(Rc::clone(start));
                } else {
                    end.borrow_mut()
                        .left
                        .as_ref()
                        .expect(
                            "if `end` is not equivalent to `start` then surely there's something \
                            to the left of `end`",
                        )
                        .borrow_mut()
                        .right = Some(Rc::clone(&old));
                    end.borrow_mut().left = Some(old);
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
                actual_elements: self.iter().count(),
            });
        };
        let Some(end) = &mut self.end else {
            self.init_empty(new);
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

    pub fn find<Q: PartialEq + ?Sized>(&self, other: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
    {
        self.iter().find(|&elem| elem.borrow() == other)
    }

    fn find_ptr<Q: PartialEq + ?Sized>(&self, other: &Q) -> Option<Rc<Inner<T>>>
    where
        T: Borrow<Q>,
    {
        self.ptr_iter()
            .find(|elem| RefCell::borrow(elem).inner.borrow().eq(other))
    }

    pub fn delete<Q: PartialEq + ?Sized>(&mut self, other: &Q) -> Option<T>
    where
        T: Borrow<Q> + Debug,
    {
        #![expect(clippy::unit_arg, reason = "I want C++ style.")]

        fn rearrange_start<T>(start: &mut Rc<Inner<T>>) -> Option<()> {
            let right = if let Some(right) = &RefCell::borrow(start).right {
                Rc::clone(right)
            } else {
                return None;
            };

            Some(*start = right)
        }
        fn rearrange_end<T>(end: &mut Rc<Inner<T>>) -> Option<()> {
            let left = if let Some(left) = &RefCell::borrow(end).left {
                Rc::clone(left)
            } else {
                return None;
            };

            Some(*end = left)
        }
        fn rearrange_left<T>(left: &Rc<Inner<T>>, target: &Inner<T>) {
            RefCell::borrow_mut(left)
                .right
                .clone_from(&RefCell::borrow(target).right);
        }
        fn rearrange_right<T>(right: &Rc<Inner<T>>, target: &Inner<T>) {
            RefCell::borrow_mut(right)
                .left
                .clone_from(&RefCell::borrow(target).left);
        }

        let target = self.find_ptr(other)?;
        let (Some(start), Some(end)) = (&mut self.start, &mut self.end) else {
            return None;
        };

        if Rc::ptr_eq(start, &target)
            && let None = rearrange_start(start)
        {
            self.start = None;
        }
        if Rc::ptr_eq(end, &target)
            && let None = rearrange_end(end)
        {
            self.end = None;
        }

        if let Some(left) = &RefCell::borrow(&target).left {
            rearrange_left(left, &target);
        }
        if let Some(right) = &RefCell::borrow(&target).right {
            rearrange_right(right, &target);
        }

        let Some(target) = Rc::into_inner(target) else {
            panic!("`target` should only have a single reference at this point");
        };

        Some(target.into_inner().inner)
    }

    #[expect(
        clippy::must_use_candidate,
        reason = "It's not a bug not to use the result of this method."
    )]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            first: self.start.as_ref().map(|elem| elem.as_ptr().cast_const()),
            last: self.end.as_ref().map(|elem| elem.as_ptr().cast_const()),
            current: None,
            _marker: PhantomData,
        }
    }

    fn ptr_iter(&self) -> PtrIter<T> {
        PtrIter {
            // The below use a method-type clone instead of an associated
            // function approach because start and end are wrapped in `Option`s.
            start: self.start.clone(),
            end: self.end.clone(),
            current: 0,
            current_indexer: None,
        }
    }
}

impl<'a, T: PartialEq + Default> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = <Iter<'a, T> as Iterator>::Item;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: PartialEq + Default> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: IntoIterator<Item: PartialEq + Default + Debug>> From<T> for DoublyLinkedList<T::Item> {
    fn from(value: T) -> Self {
        value.into_iter().fold(Self::default(), |mut accum, elem| {
            accum.insert_at(elem, InsertionPos::End);

            accum
        })
    }
}

#[derive(Clone, Copy)]
pub enum InsertionPos {
    Start,
    End,
}

#[derive(Debug)]
pub struct Node<T> {
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    inner: T,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    first: Option<*const Node<T>>,
    last: Option<*const Node<T>>,
    current: Option<*const Node<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => self.current = Some(self.first?),
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

    macro_rules! insertion_test {
        ($list:expr, $new:expr, $pos:expr, $test:expr$(,)?) => {{
            $list.insert_at($new, $pos);
            assert_eq!(
                $list.iter().map(ToString::to_string).collect::<Vec<_>>(),
                $test
            );
        }};
    }

    macro_rules! search_test {
        ($list:expr, Some($test:expr) $(,)?) => {{ assert_eq!($list.find($test), Some(&$test)) }};
        ($list:expr, None($test:expr) $(,)?) => {{ assert_eq!($list.find($test), None) }};
    }

    macro_rules! deletion_test {
        ($list:expr, Some($test:expr), $state:expr$(,)?) => {{
            assert_eq!($list.delete($test), Some($test));
            assert_eq!(
                $list.iter().map(ToString::to_string).collect::<Vec<_>>(),
                $state
            );
        }};
        ($list:expr, None($test:expr), $state:expr$(,)?) => {{
            assert_eq!($list.delete($test), None);
            assert_eq!(
                $list.iter().map(ToString::to_string).collect::<Vec<_>>(),
                $state
            );
        }};
    }

    #[test]
    fn insertion_at_start() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        insertion_test!(
            list,
            "Something else",
            InsertionPos::Start,
            ["Something else", "Something", "else"]
        );
    }

    #[test]
    fn insertion_at_end() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        insertion_test!(
            list,
            "Nothing",
            InsertionPos::End,
            ["Something", "else", "Nothing"]
        );
    }

    #[test]
    fn insertion_at_idx_correct() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        let _ = list.insert_at_idx("NUMA", 1);
        assert_eq!(
            list.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["Something", "NUMA", "else"]
        );
    }

    #[test]
    fn insertion_at_idx_incorrect() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        assert!(list.insert_at_idx("NUMA", 10).is_err_and(|err| {
            matches!(
                err,
                InsertionError::IndexOutOfBounds {
                    wrong_index: 10,
                    actual_elements: 2,
                }
            )
        }));
    }

    #[test]
    fn search_found() {
        let list = DoublyLinkedList::from(["Something", "else"]);

        search_test!(list, Some("else"));
    }

    #[test]
    fn search_not_found() {
        let list = DoublyLinkedList::from(["Something", "else"]);

        search_test!(list, None("nothing"));
    }

    #[test]
    fn deletion_found() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        deletion_test!(list, Some("Something"), ["else"]);
    }

    #[test]
    fn deletion_not_found() {
        let mut list = DoublyLinkedList::from(["Something", "else"]);

        deletion_test!(list, None("other"), ["Something", "else"]);
    }
}
