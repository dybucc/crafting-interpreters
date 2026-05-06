use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct Location {
    line: usize,
    col: usize,
    len: usize,
}

// NOTE: we provide a default implementation for the `Location` type, though the
// accessor methods we implement on  the type make it simple to overwrite this
// with some other `Display`-like implementation. This can happen most often
// when used inside errors for reporting the span of the underlying error
// location.
impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { line, col, .. } = self;

        write!(f, "{line}:{col}")
    }
}

impl Location {
    pub(crate) fn new(line: usize, col: usize, len: usize) -> Self {
        Self { line, col, len }
    }

    pub(crate) fn same_line(&self, other: &Self) -> bool {
        self.line == other.line
    }

    // NOTE: if this is used outside `SyntaxError`s, consider adding a
    // different/auxiliary routine to consider more cases of similarity between
    // spans, beyond column comparison in the same line.
    pub(crate) fn akin_col(&self, other: &Self) -> bool {
        // NOTE: example showcase of syntax error spans being merged. Locations are
        // determined to be akin if they compare equal with one byte offset difference.
        // self:  func s(s: t1, s t2:
        //                      ^^^^ expected parameter type
        // Location { line: x, col: y, len: 5 }
        // other: func s(s: t1, s: t2:
        //                           ^ unexpected char, expected ')'
        // Location { line: x, col: y + 1, len: 1 }
        // other: func s(s: t1, s t2:
        //                      ^^^^^ multiple parameters with same name
        //                            unexpected char, expected ')'
        // Location { line: x, col: y, len: 6 }

        let Location { col, len, .. } = self;
        let Location {
            col: ocol,
            len: olen,
            ..
        } = other;

        if (col + len).abs_diff(ocol + olen) <= 1 {
            return true;
        }

        false
    }

    pub(crate) fn merge_cols(&mut self, other: Self) {
        let Location { len, col, line } = self;
        let Location {
            len: olen,
            col: mut ocol,
            line: oline,
        } = other;

        debug_assert_eq!(*line, oline);

        match col.cmp(&&mut ocol) {
            Ordering::Less => *len = ocol - *col + olen,
            Ordering::Greater => {
                *col = ocol;
                *len += *col - ocol;
            }
            Ordering::Equal => *len += olen,
        }
    }

    pub(crate) fn line(&self) -> usize {
        self.line
    }

    pub(crate) fn col(&self) -> usize {
        self.col
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }
}
