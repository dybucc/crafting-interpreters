#![expect(dead_code, reason = "WIP.")]

use std::{borrow::Borrow, marker::PhantomData};

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

    fn insert(&mut self, other: T) {
        if let Some(elem) = &mut self.start {
            let new = Some(Node {
                left: None,
                right: None,
                inner: other,
            });
        } else {
            self.start = Some(Box::new(Node {
                left: None,
                right: None,
                inner: other,
            }));
        }
    }

    fn delete<U>(&mut self, other: U)
    where
        T: AsRef<U>,
    {
        todo!()
    }

    fn find(&self, other: &T) -> Option<usize> {
        todo!()
    }

    fn state(&self) -> Vec<&T> {
        self.iter().collect::<Vec<_>>()
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

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U, const N: usize> From<[U; N]> for DoublyLinkedList<T>
where
    T: Borrow<U>,
    U: ToOwned<Owned = T>,
{
    fn from(value: [U; N]) -> Self {
        let mut output = Self {
            start: None,
            end: None,
            _marker: PhantomData,
        };
        value.into_iter().for_each(|elem| {
            output.insert(elem.to_owned());
        });

        output
    }
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
        let list = DoublyLinkedList::from(["Something", "else"]);

        list.insert("Something else");
        assert_eq!(list.state(), &["Something", "else", "Something else"]);

        assert_eq!(list.find("else"), Some(1));
        assert_eq!(list.find("nothing"), None);

        assert_eq!(list.delete("Something"), String::from("Something"));
    }
}
