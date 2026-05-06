pub(crate) trait PeekerPattern {
    fn eval(&mut self) -> impl FnOnce(&u8) -> bool;
}

impl PeekerPattern for u8 {
    fn eval(&mut self) -> impl FnOnce(&u8) -> bool {
        |peeker| *peeker == *self
    }
}

impl PeekerPattern for &[u8] {
    fn eval(&mut self) -> impl FnOnce(&u8) -> bool {
        |peeker| peeker == self.first().unwrap()
    }
}

impl<F: FnMut(&u8) -> bool> PeekerPattern for F {
    fn eval(&mut self) -> impl FnOnce(&u8) -> bool {
        self
    }
}
