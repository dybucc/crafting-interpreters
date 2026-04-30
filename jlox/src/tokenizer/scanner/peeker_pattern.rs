pub(crate) trait PeekerPattern {
    fn eval(&mut self) -> impl FnOnce(&[u8]) -> bool;
}

impl<const N: usize> PeekerPattern for &[u8; N] {
    fn eval(&mut self) -> impl FnOnce(&[u8]) -> bool {
        |peeker| peeker == self.as_slice()
    }
}

impl PeekerPattern for &[u8] {
    fn eval(&mut self) -> impl FnOnce(&[u8]) -> bool {
        |peeker| peeker == *self
    }
}

impl<F: FnMut(&[u8]) -> bool> PeekerPattern for F {
    fn eval(&mut self) -> impl FnOnce(&[u8]) -> bool {
        self
    }
}
