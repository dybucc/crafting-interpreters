mod errors;
mod lexeme;
mod literal;
mod location;
mod scanner;
mod token_type;

pub(crate) use crate::tokenizer::{
    lexeme::Lexeme, literal::Lit, location::Location, token_type::TokenType,
};

// TODO: finish implementing common traits for the below types, and see into
// building some abstraction over whether a literal is meant to hold something
// or not.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Token {
    ty: TokenType,
    lex: Lexeme,
    lit: Option<Lit>,
    loc: Location,
}

impl Token {
    /// Creates a new token provided the stream of bytes from which to source
    /// the token, and the actual token type to parse.
    ///
    /// Note the token type is meant solely for tokens with a raw byte length
    /// larger than 2, as those require parsing the bytes themselves.
    pub(crate) fn new(bytes: &[u8], hint: Option<TokenType>, loc: Location) -> Self {
        debug_assert_ne!(bytes.len(), 0);

        match bytes.len() {
            1 => Self {
                ty: TokenType::single(*bytes.first().unwrap()),
                lex: Lexeme::from(bytes),
                lit: None,
                loc,
            },
            2 => Self {
                ty: TokenType::compound(bytes),
                lex: Lexeme::from(bytes),
                lit: None,
                loc,
            },
            _ if let Some(hint) = hint => Self {
                ty: hint,
                lex: Lexeme::from(bytes),
                lit: None,
                loc,
            },
            _ => panic!("tokens with length larger than 2 should include a hint on the token type"),
        }
    }
}
