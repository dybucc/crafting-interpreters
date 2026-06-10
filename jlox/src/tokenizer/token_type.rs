use std::fmt::{self, Debug, Display as StdDisplay, Formatter};

mod display;

pub(crate) use display::Display;

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub(crate) enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Ident,
    String,
    Num,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof
}

impl TokenType {
    pub(crate) fn compound(bytes: &[u8]) -> Self {
        match bytes {
            b"!=" => TokenType::BangEqual,
            b"==" => TokenType::EqualEqual,
            b">=" => TokenType::GreaterEqual,
            b"<=" => TokenType::LessEqual,
            _ => unreachable!()
        }
    }

    pub(crate) fn single(byte: u8) -> Self {
        match byte {
            b'(' => TokenType::LeftParen,
            b')' => TokenType::RightParen,
            b'{' => TokenType::LeftBrace,
            b'}' => TokenType::RightBrace,
            b',' => TokenType::Comma,
            b'.' => TokenType::Dot,
            b'-' => TokenType::Minus,
            b'+' => TokenType::Plus,
            b';' => TokenType::Semicolon,
            b'*' => TokenType::Star,
            b'!' => TokenType::Bang,
            b'=' => TokenType::Equal,
            b'>' => TokenType::Greater,
            b'<' => TokenType::Less,
            _ => unreachable!()
        }
    }

    /// Provided an external displayable type, this will return a display
    /// adapter that will use that displayable if the token type is one of a
    /// string, an idenfitfier or a number.
    ///
    /// Otherwise, the default [`Display`] impl for `TokenType` will be used
    /// instead.
    ///
    /// [`Display`]: trait@std::fmt::Display
    pub(crate) fn display_with(self, token: &impl StdDisplay) -> Display<'_> {
        Display::new(self, token)
    }
}

/// Displays the token type with lossy information for richer token types.
///
/// This implementation will correctly provide the same source code
/// representations of each token type to all tokens but strings, identifiers,
/// and numbers. These will use a fallback display implementation that is likely
/// not representative of the token itself.
///
/// To provide as well a type with which to format the afore mentioned token
/// types, see the [`TokenType::display_with`] method.
///
/// [`TokenType::display_with`]: fn@self::TokenType::display_with
impl StdDisplay for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftParen => f.pad("("),
            Self::RightParen => f.pad(")"),
            Self::LeftBrace => f.pad("{"),
            Self::RightBrace => f.pad("}"),
            Self::Comma => f.pad(","),
            Self::Dot => f.pad("."),
            Self::Minus => f.pad("-"),
            Self::Plus => f.pad("+"),
            Self::Semicolon => f.pad(";"),
            Self::Slash => f.pad("/"),
            Self::Star => f.pad("*"),
            Self::Bang => f.pad("!"),
            Self::BangEqual => f.pad("!="),
            Self::Equal => f.pad("="),
            Self::EqualEqual => f.pad("=="),
            Self::Greater => f.pad(">"),
            Self::GreaterEqual => f.pad(">="),
            Self::Less => f.pad("-"),
            Self::LessEqual => f.pad("<="),
            Self::Ident => f.pad("{ident}"),
            Self::String => f.pad("{string}"),
            Self::Num => f.pad("{num}"),
            Self::And => f.pad("and"),
            Self::Class => f.pad("class"),
            Self::Else => f.pad("else"),
            Self::False => f.pad("false"),
            Self::Fun => f.pad("fun"),
            Self::For => f.pad("for"),
            Self::If => f.pad("if"),
            Self::Nil => f.pad("nil"),
            Self::Or => f.pad("or"),
            Self::Print => f.pad("print"),
            Self::Return => f.pad("return"),
            Self::Super => f.pad("super"),
            Self::This => f.pad("this"),
            Self::True => f.pad("true"),
            Self::Var => f.pad("var"),
            Self::While => f.pad("while"),
            Self::Eof => f.pad("{eof}")
        }
    }
}
