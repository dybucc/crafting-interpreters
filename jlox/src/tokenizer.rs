mod errors;
mod lexeme;
mod literal;
mod location;
mod scanner;
mod token_type;

use std::{
    debug_assert_matches,
    fmt::{self, Display, Formatter}
};

pub(crate) use crate::tokenizer::{
    errors::SyntaxError, lexeme::Lexeme, literal::Lit, location::Location, token_type::TokenType
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Token {
    ty: TokenType,
    lex: Lexeme,
    lit: Option<Lit>,
    loc: Location
}

impl Token {
    /// Crates a byte-long token.
    pub(crate) fn single(byte: u8, loc: Location) -> Self {
        Self { ty: TokenType::single(byte), lex: Lexeme::from([byte].as_slice()), lit: None, loc }
    }

    /// Creates a two-bytes-long token.
    pub(crate) fn compound(bytes: [u8; 2], loc: Location) -> Self {
        Self {
            ty: TokenType::compound(&bytes),
            lex: Lexeme::from(bytes.as_slice()),
            lit: None,
            loc
        }
    }

    /// Creates a token whose meaning is left to the parsing facilities.
    ///
    /// This routine does not attempt to parse the token type from the provided
    /// bytes because such functionality is already implemented in the scanner.
    /// This module serves only as a bridge between the scanner and the parser,
    /// so performing another linear scan over the input bytes is not worth it.
    pub(crate) fn multiple(bytes: &[u8], token: TokenType, loc: Location) -> Self {
        debug_assert_matches!(bytes.len(), n if n > 2);

        Self { ty: token, lex: Lexeme::from(bytes), lit: None, loc }
    }
}

/// Displays the token type as seen during source code scanning.
///
/// This implementation will display the token type that was processed, and the
/// parsed lexeme of the token if this happens to be an identifier, a string, or
/// a number.
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let TokenType::Ident | TokenType::String | TokenType::Num = self.ty {
            self.ty.display_with(&self.lex).fmt(f)
        } else {
            self.ty.fmt(f)
        }
    }
}
