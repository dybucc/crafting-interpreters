#![feature(extend_one)]

use std::{
  borrow::Borrow,
  cell::RefCell,
  cmp::Ordering,
  fmt::{self, Debug, Display, Formatter},
  hash::{Hash, Hasher},
  marker::PhantomData,
  ops::ControlFlow,
  rc::Rc,
};

use itertools::Itertools;
use thiserror::Error;

type Inner<T> = RefCell<Node<T>>;

pub struct DoublyLinkedList<T> {
  start: Option<Rc<Inner<T>>>,
  end:   Option<Rc<Inner<T>>>,
  len:   usize,
}

#[derive(Error, Debug)]
pub enum InsertElemError {
  #[error(
    "passed index {wrong_index} out of bounds; only {actual_elements} \
     elements available"
  )]
  IndexOutOfBounds { wrong_index: usize, actual_elements: usize },
  #[error(
    "list is empty; elements can only be added to the list by index if it's \
     non-empty"
  )]
  EmptyList,
}

#[macro_export]
macro_rules! insert {
  ($self:expr, $other:expr) => {{
    $self.insert_at($other, InsertionPos::End);
  }};
  ($self:expr, $other:expr, $pos:expr) => {{
    $self.insert_at($other, $pos);
  }};
}

impl<T> DoublyLinkedList<T> {
  #[expect(
    clippy::must_use_candidate,
    reason = "It's not a bug for a list to be discarded."
  )]
  pub fn new() -> Self { Self { start: None, end: None, len: 0 } }

  #[expect(
    clippy::must_use_candidate,
    reason = "It's not a bug not to use the result fo this function."
  )]
  pub fn len(&self) -> usize { self.len }

  #[expect(
    clippy::must_use_candidate,
    reason = "It's not a bug not to use the result fo this function."
  )]
  pub fn is_empty(&self) -> bool { self.len == 0 }

  fn init_single_elem(&mut self, new: Node<T>) {
    let new = Rc::new(RefCell::new(new));
    self.end = Some(Rc::clone(&new));
    self.start = Some(new);
    self.len = 1;
  }

  pub fn insert_at<Q: Into<T>>(&mut self, other: Q, pos: InsertionPos) {
    let new = Node { left: None, right: None, inner: other.into() };
    match pos {
      | InsertionPos::Start => {
        let (Some(start), Some(end)) = (&self.start, &mut self.end) else {
          return self.init_single_elem(new);
        };
        let mut old = start.replace(new);
        old.left = Some(Rc::clone(start));
        let old = Rc::new(RefCell::new(old));
        if Rc::ptr_eq(start, end) {
          *end = old;
          RefCell::borrow_mut(start).right = Some(Rc::clone(end));
        } else {
          RefCell::borrow_mut(RefCell::borrow(&old).right.as_ref().expect(
            "if `start != end`, then the right of `old` ought be pointing to \
             a non-`None` as it is the older `start` and the pointers are to \
             the `RefCell`s and not to the actual `Node`s so the above check \
             is the same as `old != end`",
          ))
          .left = Some(Rc::clone(&old));

          RefCell::borrow_mut(start).right = Some(old);
        }
      },
      | InsertionPos::End => {
        let (Some(start), Some(end)) = (&mut self.start, &self.end) else {
          return self.init_single_elem(new);
        };
        let mut old = end.replace(new);
        old.right = Some(Rc::clone(end));
        let old = Rc::new(RefCell::new(old));
        if Rc::ptr_eq(start, end) {
          *start = old;
          RefCell::borrow_mut(end).left = Some(Rc::clone(start));
        } else {
          RefCell::borrow_mut(RefCell::borrow(&old).left.as_ref().expect(
            "if `start != end`, then the left of `old` ought be pointing to a \
             non-`None` as it is the older `end` and the pointers are to the \
             `RefCell`s and not to the actual `Node`s so the above check is \
             the same as `start != old`",
          ))
          .right = Some(Rc::clone(&old));

          RefCell::borrow_mut(end).left = Some(old);
        }
      },
    }
    self.len += 1;
  }

  pub fn insert_at_idx<Q: Into<T>>(
    &mut self,
    other: Q,
    idx: usize,
  ) -> Result<(), InsertElemError> {
    self.start.as_ref().ok_or(InsertElemError::EmptyList)?;
    let new = Node { left: None, right: None, inner: other.into() };
    let (ControlFlow::Break((len, elem)) | ControlFlow::Continue((len, elem))) =
      self
        .ptr_iter()
        .enumerate()
        .try_fold(None, |_, (inner_idx, ptr)| {
          if inner_idx == idx {
            ControlFlow::Break(Some((None, ptr)))
          } else {
            ControlFlow::Continue(Some((Some(inner_idx + 1), ptr)))
          }
        })
        .map_continue(|result| {
          result.expect(
            "`result` can never be `None` because the list is checked for \
             emptyness at the start of the method",
          )
        })
        .map_break(|result| {
          result.expect(
            "`result` can never be `None` because the list is checked for \
             emptyness at the start of the method",
          )
        });
    if let Some(len) = len {
      return Err(InsertElemError::IndexOutOfBounds {
        wrong_index:     idx,
        actual_elements: len,
      });
    }
    let Some(end) = &mut self.end else {
      self.init_single_elem(new);
      return Ok(());
    };
    let mut old = elem.replace(new);
    RefCell::borrow_mut(&elem).left.clone_from(&old.left);
    old.left = Some(Rc::clone(&elem));
    let old = Rc::new(RefCell::new(old));
    if let Some(left) = &RefCell::borrow(&elem).left {
      RefCell::borrow_mut(left).right = Some(Rc::clone(&elem));
    }
    if let Some(right) = &RefCell::borrow(&old).right {
      RefCell::borrow_mut(right).left = Some(Rc::clone(&old));
    }
    if Rc::ptr_eq(end, &elem) {
      *end = Rc::clone(&old);
    }
    RefCell::borrow_mut(&elem).right = Some(old);
    self.len += 1;

    Ok(())
  }

  pub fn get<Q: PartialEq + ?Sized>(&self, other: &Q) -> Option<&T>
  where
    T: Borrow<Q>,
  {
    self.iter().find(|&elem| elem.borrow() == other)
  }

  fn find_ptr<Q: PartialEq + ?Sized>(&self, other: &Q) -> Option<Rc<Inner<T>>>
  where
    T: Borrow<Q>,
  {
    self.ptr_iter().find(|elem| RefCell::borrow(elem).inner.borrow() == other)
  }

  pub fn append<L: Into<Self>>(&mut self, other: L) {
    let mut other = other.into();
    let Some(end) = &mut self.end else {
      return *self = other;
    };
    if let (Some(new_end), Some(new_start)) = (&other.end, &other.start) {
      RefCell::borrow_mut(end).right = Some(Rc::clone(new_start));
      RefCell::borrow_mut(new_start).left = Some(Rc::clone(end));
      *end = Rc::clone(new_end);
    }
    (other.start, other.end) = (None, None);
  }

  pub fn delete<Q: PartialEq + ?Sized>(&mut self, other: &Q) -> Option<T>
  where
    T: Borrow<Q>,
  {
    #![expect(clippy::unit_arg, reason = "Beauty comes at a cost.")]

    #[inline]
    fn rearrange_start<T>(start: &mut Rc<Inner<T>>) -> Option<()> {
      let right = if let Some(right) = &RefCell::borrow(start).right {
        Rc::clone(right)
      } else {
        return None;
      };

      Some(*start = right)
    }

    #[inline]
    fn rearrange_end<T>(end: &mut Rc<Inner<T>>) -> Option<()> {
      let left = if let Some(left) = &RefCell::borrow(end).left {
        Rc::clone(left)
      } else {
        return None;
      };

      Some(*end = left)
    }

    #[inline]
    fn rearrange_left<T>(left: &Rc<Inner<T>>, target: &Inner<T>) {
      RefCell::borrow_mut(left)
        .right
        .clone_from(&RefCell::borrow(target).right);
    }

    #[inline]
    fn rearrange_right<T>(right: &Rc<Inner<T>>, target: &Inner<T>) {
      RefCell::borrow_mut(right).left.clone_from(&RefCell::borrow(target).left);
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
    let target = Rc::into_inner(target)
      .expect("`target` should be isolated at this point");
    self.len -= 1;

    Some(target.into_inner().inner)
  }

  #[expect(
    clippy::must_use_candidate,
    reason = "It's not a bug not to use the result of this method."
  )]
  pub fn iter(&self) -> Iter<'_, T> {
    Iter {
      first:   self.start.as_ref().map(|elem| elem.as_ptr().cast_const()),
      current: None,
      _marker: PhantomData,
    }
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    IterMut {
      first:   self.start.as_ref().map(|elem| elem.as_ptr()),
      current: None,
      _marker: PhantomData,
    }
  }

  fn ptr_iter(&self) -> PtrIter<T> {
    PtrIter { first: self.start.clone(), current: None }
  }
}

/// Inherent implementation replacing an implementation of `Clone`, such that
/// the `T: Clone` bound is only required with the `clone()` method, and not
/// with the `clone_from()` method.
impl<T> DoublyLinkedList<T> {
  #![expect(
    clippy::should_implement_trait,
    reason = "Implementing the `Clone` trait forces the bound `T: Clone` at \
              the impl level, which constrains the uses of `clone_from()`, as \
              that method only performs a shallow copy. Specialization is not \
              an option, as `clone()` (which performs a deep copy) can't be \
              implemented without `T: Clone`"
  )]

  /// Clones the entire list allocations into a new list, and returns that new
  /// list.
  #[expect(
    clippy::return_self_not_must_use,
    reason = "It's not a bug not to use the result of this function."
  )]
  pub fn clone(&self) -> Self
  where
    T: Clone,
  {
    self.iter().map(Clone::clone).fold(Self::default(), |mut list, elem| {
      insert!(list, elem);

      list
    })
  }

  /// Clones only the pointers to each element of the list, without destroying
  /// the other list (i.e. performs a shallow copy by sharing resources.)
  pub fn clone_from(&mut self, source: &Self) {
    // If `self` is non-empty and isn't already being shared with some other
    // list, then deallocate all resources in it prior to filling it anew with
    // pointers to the `source` list.
    if let Some(current) = &mut self.start
      && Rc::strong_count(current) <= 2
    {
      let mut ptrs = Vec::new();
      while let Some(right) = &RefCell::borrow(&Rc::clone(current)).right {
        RefCell::borrow_mut(right).left = None;
        ptrs.push(Rc::clone(current));
        *current = Rc::clone(right);
      }
      self.start = None;
      self.end = None;
      // See the `Drop` impl for details on why each pointer must be dropped
      // sequentially.
      for ptr in ptrs {
        drop(ptr);
      }
    }
    self.start.clone_from(&source.start);
    self.end.clone_from(&source.end);
  }
}

impl<T: PartialEq> PartialEq for DoublyLinkedList<T> {
  fn eq(&self, other: &Self) -> bool { self.iter().eq(other) }
}

impl<T: Eq> Eq for DoublyLinkedList<T> {}

impl<T: PartialOrd> PartialOrd for DoublyLinkedList<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.iter().partial_cmp(other)
  }
}

impl<T: Ord> Ord for DoublyLinkedList<T> {
  fn cmp(&self, other: &Self) -> Ordering { self.iter().cmp(other) }
}

#[cfg(all(feature = "hash_elem", not(feature = "hash_ptr")))]
impl<T: Hash> Hash for DoublyLinkedList<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.iter().for_each(|elem| elem.hash(state));
  }
}

#[cfg(all(feature = "hash_ptr", not(feature = "hash_elem")))]
impl<T> Hash for DoublyLinkedList<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.ptr_iter().map(|ptr| Rc::as_ptr(&ptr)).for_each(|ptr| ptr.hash(state));
  }
}

impl<T: Debug> Debug for DoublyLinkedList<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    self.iter().try_for_each(|elem| write!(f, "{elem:?}"))
  }
}

impl<T: Display> Display for DoublyLinkedList<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    self.iter().try_for_each(|elem| write!(f, "{elem}"))
  }
}

impl<T> Drop for DoublyLinkedList<T> {
  fn drop(&mut self) {
    let Some(current) = &mut self.start else {
      return;
    };
    let mut ptrs = Vec::new();
    while let Some(right) = &RefCell::borrow(&Rc::clone(current)).right {
      RefCell::borrow_mut(right).left = None;
      ptrs.push(Rc::clone(current));
      *current = Rc::clone(right);
    }
    self.start = None;
    self.end = None;
    cfg_select! {
        debug_assertions => {
          assert!(ptrs.into_iter().enumerate().all(|(idx, ptr)| {
            let strong_count = Rc::strong_count(&ptr);
            eprintln!(
              "element idx: {idx}\nelement strong count: {strong_count}"
            );

            strong_count == 1
          }));
        }
        _ => {
          // Elements are dropped sequentially because an invariant holds such
          // that, at this point, each element has only two pointers pointing to
          // it at any time; The pointer in the vector that we are traversing,
          // and the pointer from the prior element in the vector. Thus,
          // dropping each element in serial allows to have only a single
          // pointer to the currently iterated-over pointer in the vector. This
          // is possible because there is only a single pointer to the first
          // element of the list/vector; The first element of the vector itself.
          for ptr in ptrs {
            drop(ptr);
          }
        }
    }
  }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
  type IntoIter = IntoIter<T>;
  type Item = T;

  fn into_iter(mut self) -> Self::IntoIter {
    let first = self.start.as_ref().map(Rc::clone);
    let last = self.end.as_ref().map(Rc::clone);
    self.start = None;
    self.end = None;

    IntoIter { first, last, next_ptr: None }
  }
}

impl<'a, T: 'a> IntoIterator for &'a DoublyLinkedList<T> {
  type IntoIter = Iter<'a, T>;
  type Item = <Iter<'a, T> as Iterator>::Item;

  fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, T: 'a> IntoIterator for &'a mut DoublyLinkedList<T> {
  type IntoIter = IterMut<'a, T>;
  type Item = <IterMut<'a, T> as Iterator>::Item;

  fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<T> Default for DoublyLinkedList<T> {
  fn default() -> Self { Self::new() }
}

impl<T, A: Into<T>> FromIterator<A> for DoublyLinkedList<T> {
  fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
    iter.into_iter().map_into::<T>().fold(Self::default(), |mut list, elem| {
      insert!(list, elem);

      list
    })
  }
}

impl<T, A: Into<T>> Extend<A> for DoublyLinkedList<T> {
  fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
    self.append(Self::from_iter(iter));
  }

  fn extend_one(&mut self, item: A) {
    insert!(self, item);
  }
}

#[derive(Debug, Clone, Copy)]
pub enum InsertionPos {
  Start,
  End,
}

#[derive(Debug)]
struct Node<T> {
  left:  Option<Rc<Inner<T>>>,
  right: Option<Rc<Inner<T>>>,
  inner: T,
}

pub struct IntoIter<T> {
  first:    Option<Rc<Inner<T>>>,
  last:     Option<Rc<Inner<T>>>,
  next_ptr: Option<Rc<Inner<T>>>,
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    'evaluator: {
      match self.next_ptr.as_mut() {
        | None => {
          let first = Rc::clone(self.first.as_ref()?);
          if let Some(right) = &RefCell::borrow(&first).right {
            RefCell::borrow_mut(right).left = None;
            self.next_ptr = Some(Rc::clone(right));
          }
          self.first = None;

          Some(
            Rc::into_inner(first)
              .expect("the first element of the list should be isolated here"),
          )
        },
        // It's worth it *not* to keep an exclusive reference to the option's
        // generic parameter so that we can mutate both the underlying value
        // with `unwrap()` and the whole option through field projection on
        // `self`.
        | next_option @ Some(_) => {
          let current_ptr = Rc::clone(next_option.as_ref().unwrap());
          if Rc::ptr_eq(
            &current_ptr,
            self.last.as_ref().expect(
              "if we've reached the point where there was a viable next \
               pointer, then surely there's an end item in the list",
            ),
          ) {
            (self.next_ptr, self.last) = (None, None);
            break 'evaluator Some(
              Rc::into_inner(current_ptr)
                .expect("the current element in the list should be isolated"),
            );
          }
          let next = RefCell::borrow(&current_ptr).right.clone().expect(
            "if `current_ptr` does not point to the same allocation as \
             `self.last`, then surely there's some list item to its left",
          );
          RefCell::borrow_mut(&next).left = None;
          *next_option.unwrap() = next;

          Some(
            Rc::into_inner(current_ptr)
              .expect("the current element in the list should be isolated"),
          )
        },
      }
    }
    .map(|current| current.into_inner().inner)
  }
}

#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
  first:   Option<*const Node<T>>,
  current: Option<*const Node<T>>,
  _marker: PhantomData<&'a T>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.current {
      | None => self.current = Some(self.first?),
      | Some(current) => {
        // SAFETY: the pointer is never null because it's gated behind an
        // `Option` and that `Option` is only ever `Some(_)` when there's at
        // least one element in the list (see the above `None` branch.) State
        // beyond this relies on there being a pointer to the right of the
        // current one, for which the let-else here ensures is sound.
        let Some(next) = (unsafe { &current.as_ref_unchecked().right }) else {
          return None;
        };
        self.current = Some(next.as_ptr().cast_const());
      },
    }

    self.current.map(|elem| unsafe { &elem.as_ref_unchecked().inner })
  }
}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
  first:   Option<*mut Node<T>>,
  current: Option<*mut Node<T>>,
  _marker: PhantomData<&'a mut T>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.current {
      | None => self.current.clone_from(&self.first),
      | Some(current) => {
        // SAFETY: the pointer is never null because it's gated behind an
        // `Option` and that `Option` is only ever `Some(_)` when there's at
        // least one element in the list (see the above `None` branch.) State
        // beyond this relies on there being a pointer to the right of the
        // current one, for which the let-else here ensures is sound.
        let Some(right) = (unsafe { &current.as_ref_unchecked().right }) else {
          return None;
        };
        self.current = Some(right.as_ptr());
      },
    }

    // SAFETY: if execution flow has reached this point, then the pointer can
    // only ever be wrapped `Some` for the reasons mentioned in the above SAFETY
    // comment.
    self.current.map(|ptr| unsafe { &mut ptr.as_mut_unchecked().inner })
  }
}

#[derive(Debug)]
struct PtrIter<T> {
  first:   Option<Rc<Inner<T>>>,
  current: Option<Rc<Inner<T>>>,
}

impl<T> Iterator for PtrIter<T> {
  type Item = Rc<Inner<T>>;

  fn next(&mut self) -> Option<Self::Item> {
    let Some(start) = &self.first else {
      return None;
    };
    match self.current {
      | None => self.current = Some(Rc::clone(start)),
      | Some(ref mut indexer)
        if let Some(ref right) = RefCell::borrow(&Rc::clone(indexer)).right =>
      {
        *indexer = Rc::clone(right);
      },
      | _ => self.current = None,
    }

    self.current.as_ref().map(Rc::clone)
  }
}

#[cfg(test)]
mod tests {
  use std::{assert_matches, ops::Deref, string::ToString};

  use super::*;

  macro_rules! insertion_test {
    ($list:expr, $new:expr, $pos:expr, $test:expr $(,)?) => {{
      $list.insert_at($new, $pos);
      assert_eq!(
        $list.iter().map(ToString::to_string).collect::<Vec<_>>(),
        $test
      );
    }};
  }

  macro_rules! search_test {
    ($list:expr,Some($test:expr) $(,)?) => {{ assert_eq!($list.get($test), Some(&$test)) }};
    ($list:expr,None($test:expr) $(,)?) => {{ assert_eq!($list.get($test), None) }};
  }

  macro_rules! deletion_test {
    ($list:expr,Some($test:expr), $state:expr $(,)?) => {{
      assert_eq!($list.delete($test), Some($test));
      assert_eq!(
        $list.iter().map(ToString::to_string).collect::<Vec<_>>(),
        $state
      );
    }};
    ($list:expr,None($test:expr), $state:expr $(,)?) => {{
      assert_eq!($list.delete($test), None);
      assert_eq!(
        $list.iter().map(ToString::to_string).collect::<Vec<_>>(),
        $state
      );
    }};
  }

  #[test]
  fn insertion_at_start() {
    let mut list: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["Something", "else"]);
    assert_eq!(list.len(), 2);
    insertion_test!(list, "Something else", InsertionPos::Start, [
      "Something else", "Something", "else"
    ]);
    assert_eq!(list.len(), 3);
  }

  #[test]
  fn insertion_at_end() {
    let mut list: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["Something", "else"]);
    assert_eq!(list.len(), 2);
    insertion_test!(list, "Nothing", InsertionPos::End, [
      "Something", "else", "Nothing"
    ]);
    assert_eq!(list.len(), 3);
  }

  #[test]
  fn insertion_at_idx_empty_list() {
    let mut list: DoublyLinkedList<&str> = DoublyLinkedList::default();
    assert_eq!(list.len(), 0);
    assert_matches!(
      list.insert_at_idx("Something", 0),
      Err(InsertElemError::EmptyList)
    );
    assert_eq!(list.len(), 0);
  }

  #[test]
  fn insertion_at_idx_correct() {
    let mut list: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["Something", "else"]);
    assert_eq!(list.len(), 2);
    let _ = list.insert_at_idx("NUMA", 1);
    assert_eq!(list.len(), 3);
    assert_eq!(list.iter().map(ToString::to_string).collect::<Vec<_>>(), [
      "Something", "NUMA", "else"
    ]);
  }

  #[test]
  fn insertion_at_idx_incorrect() {
    let mut list: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["Something", "else"]);
    assert_eq!(list.len(), 2);
    assert_matches!(
      list.insert_at_idx("NUMA", 10),
      Err(InsertElemError::IndexOutOfBounds {
        wrong_index:     10,
        actual_elements: 2,
      })
    );
    assert_eq!(list.len(), 2);
  }

  #[test]
  fn search_found() {
    let list = DoublyLinkedList::from_iter(["Something", "else"]);
    search_test!(list, Some("else"));
  }

  #[test]
  fn search_not_found() {
    let list: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["Something", "else"]);
    search_test!(list, None("nothing"));
  }

  #[test]
  fn deletion_found() {
    let mut list = DoublyLinkedList::from_iter(["Something", "else"]);
    assert_eq!(list.len(), 2);
    deletion_test!(list, Some("Something"), ["else"]);
    assert_eq!(list.len(), 1);
  }

  #[test]
  fn deletion_not_found() {
    let mut list: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["Something", "else"]);
    assert_eq!(list.len(), 2);
    deletion_test!(list, None("other"), ["Something", "else"]);
    assert_eq!(list.len(), 2);
  }

  #[test]
  fn append_two_lists() {
    let mut list: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["Something", "else"]);
    itertools::assert_equal(list.iter().map(Deref::deref), [
      "Something", "else",
    ]);
    list.extend(["nothing", "else"]);
    itertools::assert_equal(list.iter().map(Deref::deref), [
      "Something", "else", "nothing", "else",
    ]);
  }

  #[test]
  fn append_multiple_lists() {
    let mut list: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["Something", "else"]);
    itertools::assert_equal(list.iter().map(Deref::deref), [
      "Something", "else",
    ]);
    list.extend(["nothing", "else"]);
    itertools::assert_equal(list.iter().map(Deref::deref), [
      "Something", "else", "nothing", "else",
    ]);
    list.extend(["nothing", "else"]);
    itertools::assert_equal(list.iter().map(Deref::deref), [
      "Something", "else", "nothing", "else", "nothing", "else",
    ]);
  }

  #[test]
  fn append_empty_list() {
    let mut list: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["Something", "else"]);
    itertools::assert_equal(list.iter().map(Deref::deref), [
      "Something", "else",
    ]);
    list.extend::<[&str; 0]>([]);
    itertools::assert_equal(list.iter().map(Deref::deref), [
      "Something", "else",
    ]);
  }

  #[test]
  fn clone_from_empty_list() {
    let (mut list1, list2): (DoublyLinkedList<&str>, DoublyLinkedList<&str>) = (
      DoublyLinkedList::from_iter::<[&str; _]>([]),
      DoublyLinkedList::from_iter(["one", "other"]),
    );
    itertools::assert_equal::<_, [&str; _]>(list1.iter().map(Deref::deref), []);
    itertools::assert_equal(list2.iter().map(Deref::deref), ["one", "other"]);
    list1.clone_from(&list2);
    itertools::assert_equal(list1.iter().map(Deref::deref), ["one", "other"]);
    itertools::assert_equal(list2.iter().map(Deref::deref), ["one", "other"]);
    // Ensures the destructor for `list2` runs properly by getting rid of the
    // other references to its first and last element.
    list1.start = None;
    list1.end = None;
  }

  #[test]
  fn clone_empty_list() {
    let (mut list1, list2): (DoublyLinkedList<&str>, DoublyLinkedList<&str>) = (
      DoublyLinkedList::from_iter(["one", "other"]),
      DoublyLinkedList::from_iter::<[&str; _]>([]),
    );
    itertools::assert_equal(list1.iter().map(Deref::deref), ["one", "other"]);
    itertools::assert_equal::<_, [&str; _]>(list2.iter().map(Deref::deref), []);
    list1.clone_from(&list2);
    itertools::assert_equal::<_, [&str; _]>(list1.iter().map(Deref::deref), []);
    itertools::assert_equal::<_, [&str; _]>(list2.iter().map(Deref::deref), []);
  }

  #[test]
  fn clone_non_empty_lists() {
    let (mut list1, list2): (DoublyLinkedList<&str>, DoublyLinkedList<&str>) = (
      DoublyLinkedList::from_iter(["Something", "else"]),
      DoublyLinkedList::from_iter(["other", "one"]),
    );
    itertools::assert_equal::<_, [&str; _]>(list1.iter().map(Deref::deref), [
      "Something", "else",
    ]);
    itertools::assert_equal::<_, [&str; _]>(list2.iter().map(Deref::deref), [
      "other", "one",
    ]);
    list1.clone_from(&list2);
    itertools::assert_equal::<_, [&str; _]>(list1.iter().map(Deref::deref), [
      "other", "one",
    ]);
    itertools::assert_equal::<_, [&str; _]>(list2.iter().map(Deref::deref), [
      "other", "one",
    ]);
    // Ensures the destructor for `list2` runs properly by getting rid of the
    // other references to its first and last element.
    list1.start = None;
    list1.end = None;
  }

  #[test]
  fn deep_clone() {
    let list1: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter(["something", "else"]);
    itertools::assert_equal::<_, [&str; _]>(list1.iter().map(Deref::deref), [
      "something", "else",
    ]);
    let list2 = list1.clone();
    itertools::assert_equal::<_, [&str; _]>(list1.iter().map(Deref::deref), [
      "something", "else",
    ]);
    itertools::assert_equal::<_, [&str; _]>(list2.iter().map(Deref::deref), [
      "something", "else",
    ]);
  }

  #[test]
  fn deep_clone_empty_list() {
    let list1: DoublyLinkedList<&str> =
      DoublyLinkedList::from_iter::<[&str; _]>([]);
    itertools::assert_equal::<_, [&str; _]>(list1.iter().map(Deref::deref), []);
    let list2 = list1.clone();
    itertools::assert_equal::<_, [&str; _]>(list1.iter().map(Deref::deref), []);
    itertools::assert_equal::<_, [&str; _]>(list2.iter().map(Deref::deref), []);
  }

  // NOTE:
  // The below tests only check for the iterators to not be doing any funky
  // stuff with their allocations, and so it only makes sense to run them with
  // Miri, so as to get better diagnostics.

  #[cfg(miri)]
  mod miri {
    use super::*;

    impl<'a, T: 'a> DoublyLinkedList<T> {
      // Doesn't use the list's own iterators because the whole point of this
      // method is to check the state of the list across calls to its own
      // iterator methods.
      fn state(&'a self) -> Vec<&'a T> {
        let mut current = self.start.as_ref().map(Rc::clone).expect(
          "the list should be non-empty in tests that use this function",
        );
        let mut out = Vec::with_capacity(self.len);
        out.push(&raw const RefCell::borrow(&current).inner);
        while let Some(right) = &RefCell::borrow(&Rc::clone(&current)).right {
          out.push(&raw const RefCell::borrow(right).inner);
          current = Rc::clone(right);
        }

        // SAFETY: the pointers are to elements allocated in the list, so
        // retreving a reference to the underlying value is sound with the given
        // method lifetime bounds.
        out.into_iter().map(|ptr| unsafe { ptr.as_ref_unchecked() }).collect()
      }
    }

    #[test]
    fn consuming_iter() {
      let list: DoublyLinkedList<&str> =
        DoublyLinkedList::from_iter(["Something", "else"]);
      itertools::assert_equal(
        list.state().as_array::<2>().unwrap().map(Deref::deref),
        ["Something", "else"],
      );
      for elem in list {
        dbg!(elem);
      }
    }

    #[test]
    fn shared_iter() {
      let list: DoublyLinkedList<&str> =
        DoublyLinkedList::from_iter(["Something", "else"]);
      itertools::assert_equal(
        list.state().as_array::<2>().unwrap().map(Deref::deref),
        ["Something", "else"],
      );
      for elem in &list {
        dbg!(elem);
      }
      itertools::assert_equal(
        list.state().as_array::<2>().unwrap().map(Deref::deref),
        ["Something", "else"],
      );
    }

    #[test]
    fn exclusive_iter() {
      let mut list: DoublyLinkedList<&str> =
        DoublyLinkedList::from_iter(["Something", "else"]);
      itertools::assert_equal(
        list.state().as_array::<2>().unwrap().map(Deref::deref),
        ["Something", "else"],
      );
      for elem in &mut list {
        dbg!(elem);
      }
      itertools::assert_equal(
        list.state().as_array::<2>().unwrap().map(Deref::deref),
        ["Something", "else"],
      );
    }
  }
}
