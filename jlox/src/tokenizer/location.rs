#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct Location {
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) len: usize,
}

impl Location {
    pub(crate) fn new(line: usize, col: usize, len: usize) -> Self {
        Self { line, col, len }
    }
}
