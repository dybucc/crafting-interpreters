use crate::tokenizer::{Location, Token, TokenType};

// TODO: finish the below interface.

#[derive(Debug, Default)]
pub(crate) struct TokenBuilder {
    repr: TokenRepr,
}

impl TokenBuilder {
    pub(crate) fn finalize(&mut self) -> Token {
        let Self { repr } = self;

        repr.finalize()
    }

    pub(crate) fn single(&mut self, bytes: &[u8], loc: Location) -> &mut Self {
        let Self { repr } = self;

        match repr.kind() {
            TokenKind::Empty => *repr = TokenRepr::single(bytes, loc),
            TokenKind::Configuration => repr.recycle(bytes, loc),
        }

        self
    }

    pub(crate) fn compound(&mut self, bytes: &[u8], loc: Location) -> &mut Self {
        let Self { repr } = self;

        match repr.kind() {
            TokenKind::Empty => *repr = TokenRepr::compound(bytes, loc),
            TokenKind::Configuration => repr.recycle(bytes, loc),
        }

        self
    }

    pub(crate) fn multiple(&mut self, bytes: &[u8], hint: TokenType, loc: Location) -> &mut Self {
        let Self { repr } = self;

        match repr.kind() {
            TokenKind::Empty => *repr = TokenRepr::multiple(bytes, hint, loc),
            TokenKind::Configuration => repr.recycle(bytes, loc),
        }

        self
    }
}

#[derive(Debug, Clone, Copy)]
enum TokenKind {
    Empty,
    Configuration,
}

#[derive(Debug, Default)]
enum TokenRepr {
    #[default]
    Empty,
    Configuration(TokenConfigurator),
}

impl TokenRepr {
    fn recycle_single(&mut self, bytes: &[u8], loc: Location) {
        match self {
            Self::Empty => panic!("configurator did not contain enough information to build token"),
            Self::Configuration(token_configurator) => {
                token_configurator.recycle_single(bytes, loc)
            }
        }
    }

    fn recycle_compound(&mut self, bytes: &[u8], loc: Location) {
        match self {
            Self::Empty => panic!("configurator did not contain enough information to build token"),
            Self::Configuration(token_configurator) => {
                token_configurator.recycle_compound(bytes, loc)
            }
        }
    }

    fn recycle_multiple(&mut self, bytes: &[u8], hint: TokenType, loc: Location) {
        match self {
            Self::Empty => panic!("configurator did not contain enough information to build token"),
            Self::Configuration(token_configurator) => {
                token_configurator.recycle_multiple(bytes, hint, loc)
            }
        }
    }

    fn finalize(&mut self) -> Token {
        match self {
            Self::Empty => {
                panic!("configurator did not contain enough information to build a token")
            }
            Self::Configuration(token_configurator) => token_configurator.finalize(),
        }
    }

    fn single(bytes: &[u8], loc: Location) -> Self {
        Self::Configuration(TokenConfigurator::single(
            bytes.first().copied().unwrap(),
            loc,
        ))
    }

    fn compound(bytes: &[u8], loc: Location) -> Self {
        Self::Configuration(TokenConfigurator::compound(bytes, loc))
    }

    fn multiple(bytes: &[u8], hint: TokenType, loc: Location) -> Self {
        Self::Configuration(TokenConfigurator::multiple(bytes, hint, loc))
    }

    fn kind(&self) -> TokenKind {
        match self {
            Self::Empty => TokenKind::Empty,
            Self::Configuration(_) => TokenKind::Configuration,
        }
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
}

impl TokenConfigurator {
    fn recycle_single(&mut self, byte: u8, loc: Location) {
        let Self {
            token_type,
            loc: self_loc,
        } = self;

        *token_type = TokenType::single(byte);
        *self_loc = loc;
    }

    fn recycle_compound(&mut self, bytes: &[u8], loc: Location) {
        let Self {
            token_type,
            loc: self_loc,
        } = self;

        *token_type = TokenType::compound(byte);
        *self_loc = loc;
    }

    fn finalize(&mut self) -> Token {
        todo!()
    }

    fn multiple(bytes: &[u8], token: TokenType, loc: Location) -> Self {
        todo!()
    }

    fn compound(bytes: &[u8], loc: Location) -> Self {
        Self {
            token_type: TokenType::compound(bytes),
            loc,
        }
    }

    fn single(byte: u8, loc: Location) -> Self {
        Self {
            token_type: TokenType::single(byte),
            loc,
        }
    }
}
