use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct Location {
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) len: usize,
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

    #[inline]
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
}
