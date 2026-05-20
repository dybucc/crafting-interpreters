use crate::tokenizer::{Lexeme, Lit, Location, Token, TokenType};

// TODO: finish the below interface.

#[derive(Debug, Default)]
pub(crate) struct TokenBuilder {
    repr: Option<TokenConfigurator>,
}

impl TokenBuilder {
    pub(crate) fn finalize(&mut self) -> Token {
        let TokenBuilder { repr } = self;

        repr.finalize()
    }

    pub(crate) fn with_lexeme(&mut self, lex: impl AsRef<[u8]>) -> &mut Self {
        let TokenBuilder { repr } = self;

        repr.add_lexeme(lex);

        self
    }

    pub(crate) fn single(&mut self, bytes: &[u8], loc: Location) -> &mut Self {
        let TokenBuilder { repr } = self;

        debug_assert_eq!(
            bytes.len(),
            1,
            "bytes in single token constructor should be made up of a single raw byte"
        );

        if let Some(config) = repr {
            config.recycle_single(*bytes.first().unwrap(), loc)
        } else {
            *repr = TokenConfigurator::single(*bytes.first().unwrap(), loc).into()
        }

        self
    }

    pub(crate) fn compound(&mut self, bytes: &[u8], loc: Location) -> &mut Self {
        let TokenBuilder { repr } = self;

        if let Some(config) = repr {
            config.recycle_compound(bytes, loc);
        } else {
            *repr = TokenConfigurator::compound(bytes, loc).into();
        }

        self
    }

    pub(crate) fn multiple(&mut self, bytes: &[u8], hint: TokenType, loc: Location) -> &mut Self {
        let TokenBuilder { repr } = self;

        if let Some(config) = repr {
            config.recycle_multiple(bytes, hint, loc);
        } else {
            *repr = TokenConfigurator::multiple(bytes, hint, loc).into();
        }

        self
    }
}

// NOTE: on construction and subsequent calls before producing the final `Token`
// instance, we only save information related to the token type (as that must be
// fetched from the set of bytes we parse, and on the location of that potential
// token.)
#[derive(Debug)]
struct TokenConfigurator {
    token_type: TokenType,
    loc: Location,
    rest: Delayed,
}

impl TokenConfigurator {
    fn recycle_single(&mut self, byte: u8, loc: Location) {
        let Self {
            token_type,
            loc: self_loc,
            ..
        } = self;

        *token_type = TokenType::single(byte);
        *self_loc = loc;
    }

    fn recycle_compound(&mut self, bytes: &[u8], loc: Location) {
        let Self {
            token_type,
            loc: self_loc,
            ..
        } = self;

        *token_type = TokenType::compound(bytes);
        *self_loc = loc;
    }

    fn recycle_multiple(&mut self, bytes: &[u8], token: TokenType, loc: Location) {
        todo!()
    }

    fn finalize(&mut self) -> Token {
        todo!()
    }

    fn multiple(bytes: &[u8], token: TokenType, loc: Location) -> Self {
        Self {
            token_type: token,
            loc,
            rest: match token {
                TokenType::Ident => Delayed::new(bytes),
                TokenType::String => Delayed::with_lit_str(bytes),
                TokenType::Num => Delayed::with_lit_num(bytes),
                TokenType::False => Delayed::with_lit_false(bytes),
                TokenType::True => Delayed::with_lit_true(bytes),
                _ => Delayed::new(bytes),
            },
        }
    }

    fn compound(bytes: &[u8], loc: Location) -> Self {
        Self {
            token_type: TokenType::compound(bytes),
            loc,
            rest: Delayed::default(),
        }
    }

    fn single(byte: u8, loc: Location) -> Self {
        Self {
            token_type: TokenType::single(byte),
            loc,
            rest: Delayed::default(),
        }
    }
}

#[derive(Debug, Default)]
struct Delayed {
    lexeme: Option<Lexeme>,
    lit: Option<Lit>,
}

impl Delayed {
    fn with_lit_str(bytes: impl AsRef<[u8]>) -> Self {
        Self {
            lexeme: Lexeme::from(bytes.as_ref()).into(),
            lit: Lit::from(bytes.as_ref()).into(),
        }
    }

    fn with_lit_num(bytes: impl AsRef<[u8]>) -> Self {
        todo!()
    }

    fn with_lit_false(bytes: impl AsRef<[u8]>) -> Self {
        todo!()
    }

    fn with_lit_true(bytes: impl AsRef<[u8]>) -> Self {
        todo!()
    }

    fn new(bytes: impl AsRef<[u8]>) -> Self {
        Self {
            lexeme: Lexeme::from(bytes.as_ref()).into(),
            lit: None,
        }
    }
}
