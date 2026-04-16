#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct Location {
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) len: usize,
}
