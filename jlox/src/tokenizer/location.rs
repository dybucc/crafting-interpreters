use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter}
};

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Location {
    line: usize,
    col: usize,
    len: usize
}

impl Location {
    #[inline]
    pub(crate) fn new(line: usize, col: usize, len: usize) -> Self { Self { line, col, len } }

    #[inline]
    pub(crate) fn same_line(self, other: Self) -> bool { self.line == other.line }

    /// compares two locations for similarity based off of their current
    /// positions in a line.
    ///
    /// this does not necessarily mean that both locations must be positioned on
    /// the same line, but that is often the case.
    #[inline]
    pub(crate) fn akin_col(self, other: Self) -> bool {
        // NOTE: example showcase of syntax error spans being merged. locations are
        // determined to be akin if they compare equal within one byte offset
        // difference.
        // self:  func s(s: t1, s t2:
        //                      ^^^^ expected parameter type
        // location { line: x, col: y, len: 4 }
        // other: func s(s: t1, s: t2:
        //                           ^ unexpected char, expected ')'
        // location { line: x, col: y + 4, len: 1 }
        // other: func s(s: t1, s t2:
        //                      ^^^^^ expected parameter type
        //                            unexpected char, expected ')'
        // location { line: x, col: y, len: 5 }
        (self.col + self.len).abs_diff(other.col + other.len) <= 1
    }

    pub(crate) fn merge_cols(&mut self, other: Self) {
        debug_assert_eq!(self.line, other.line);

        match self.col.cmp(&other.col) {
            Ordering::Less => self.len = other.col - self.col + other.len,
            Ordering::Greater => {
                self.col = other.col;
                self.len += self.col - other.col;
            }
            Ordering::Equal => self.len += other.len
        }
    }

    #[inline]
    pub(crate) fn line(self) -> usize { self.line }

    #[inline]
    pub(crate) fn col(self) -> usize { self.col }

    #[inline]
    pub(crate) fn len(self) -> usize { self.len }
}

/// this implementation serves as a safe default, with a format {line}:{col}.
///
/// this may be overwritten in display implemenations of wrapping types through
/// the provided accessors on the location type.
impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{}:{}", self.line, self.col) }
}
