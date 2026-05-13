mod builder;
mod errors;
mod lexeme;
mod literal;
mod location;
mod scanner;
mod token_type;

use std::sync::{LazyLock, Mutex};

pub(crate) use crate::tokenizer::{
    builder::TokenBuilder, lexeme::Lexeme, literal::Lit, location::Location, token_type::TokenType,
};

static BUILDER: Mutex<LazyLock<TokenBuilder>> = Mutex::new(LazyLock::new(TokenBuilder::default));

#[derive(Debug)]
pub(crate) struct Token {
    ty: TokenType,
    lex: Lexeme,
    lit: Option<Lit>,
    loc: Location,
}

impl Token {
    pub(crate) fn new(bytes: &[u8], hint: Option<TokenType>, loc: Location) -> Self {
        debug_assert_ne!(bytes.len(), 0);

        let mut builder = BUILDER.lock().unwrap();

        match bytes.len() {
            1 => builder.single(bytes, loc).finalize(),
            2 => builder.compound(bytes, loc).finalize(),
            _ if let Some(hint) = hint => builder.multiple(bytes, hint, loc).finalize(),
            _ => unimplemented!(),
        }
    }
}
