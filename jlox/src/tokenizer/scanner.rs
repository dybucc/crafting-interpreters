use std::io::{BufReader, Cursor};

use crate::tokenizer::{Token, scanner::peeker_pattern::PeekerPattern};

mod peeker_pattern;

#[derive(Debug)]
pub(crate) struct Scanner<'a> {
    buf: BufReader<Cursor<&'a [u8]>>,
    line: usize,
    col: usize
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self { todo!() }
}

impl Scanner<'_> {
    pub(crate) fn line(&self) -> usize { todo!() }

    pub(crate) fn col(&self) -> usize { todo!() }

    pub(crate) fn advance(&mut self, buf: &mut [u8; 1]) -> Result<bool, anyhow::Error> { todo!() }

    pub(crate) fn peek(&mut self, mut pat: impl PeekerPattern) -> Result<bool, anyhow::Error> {
        todo!()
    }

    pub(crate) fn scan(&mut self) -> Result<Vec<Token>, anyhow::Error> { todo!() }
}
