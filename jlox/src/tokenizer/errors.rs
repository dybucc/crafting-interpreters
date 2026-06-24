use std::{error::Error, fmt::Debug};

mod syntax_error;

pub(crate) use self::syntax_error::SyntaxError;

#[derive(Debug)]
pub(crate) struct IoBound {
    pub(crate) inner: Box<dyn Error + Send + Sync>,
    pub(crate) line: usize
}
