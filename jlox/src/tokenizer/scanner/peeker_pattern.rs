pub(crate) trait PeekerPattern {
    fn eval(&mut self) -> impl FnOnce(&u8) -> bool;
}
