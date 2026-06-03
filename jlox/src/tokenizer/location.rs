use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Location {
    line: usize,
    col: usize,
    len: usize,
}

impl Location {
    #[inline]
    pub(crate) fn new(line: usize, col: usize, len: usize) -> Self {
        Self { line, col, len }
    }

    #[inline]
    pub(crate) fn same_line(&self, other: &Self) -> bool {
        self.line == other.line
    }

    // NOTE: if this is used outside `SyntaxError`s, consider adding a
    // different/auxiliary routine to consider more cases of similarity between
    // spans, beyond column comparison in the same line.
    #[inline]
    pub(crate) fn akin_col(&self, other: &Self) -> bool {
        // NOTE: example showcase of syntax error spans being merged. Locations are
        // determined to be akin if they compare equal within one byte offset
        // difference.
        // self:  func s(s: t1, s t2:
        //                      ^^^^ expected parameter type
        // Location { line: x, col: y, len: 5 }
        // other: func s(s: t1, s: t2:
        //                           ^ unexpected char, expected ')'
        // Location { line: x, col: y + 1, len: 1 }
        // other: func s(s: t1, s t2:
        //                      ^^^^^ expected parameter type
        //                            unexpected char, expected ')'
        // Location { line: x, col: y, len: 6 }

        if (self.col + self.len).abs_diff(other.col + other.len) <= 1 {
            return true;
        }

        false
    }

    pub(crate) fn merge_cols(&mut self, other: Self) {
        debug_assert_eq!(self.line, other.line);

        match self.col.cmp(&other.col) {
            Ordering::Less => self.len = other.col - self.col + other.len,
            Ordering::Greater => {
                self.col = other.col;
                self.len += self.col - other.col;
            }
            Ordering::Equal => self.len += other.len,
        }
    }

    #[inline]
    pub(crate) fn line(&self) -> usize {
        self.line
    }

    #[inline]
    pub(crate) fn col(&self) -> usize {
        self.col
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.len
    }
}

/// This implementation serves as a safe default, with a format `{line}:{col}`.
///
/// This may be overwritten in `Display` implemenations of wrapping types
/// through the provided accessors on the `Location` type.
impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { line, col, .. } = self;

        write!(f, "{line}:{col}")
    }
}
