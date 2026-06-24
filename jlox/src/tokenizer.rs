mod errors;
mod lexeme;
mod literal;
mod location;
mod scanner;
mod token_type;

pub(crate) use crate::tokenizer::{
    lexeme::Lexeme, literal::Lit, location::Location, token_type::TokenType
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Token {
    ty: TokenType,
    lex: Lexeme,
    lit: Option<Lit>,
    loc: Location
}

impl Token {
    pub(crate) fn single(byte: u8, loc: Location) -> Self { todo!() }

    pub(crate) fn compound(bytes: [u8; 2], loc: Location) -> Self { todo!() }

    pub(crate) fn multiple(bytes: &[u8], token: TokenType, loc: Location) -> Self { todo!() }
}
