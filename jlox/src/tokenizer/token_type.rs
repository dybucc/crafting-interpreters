use std::fmt::{self, Display, Formatter};

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
    Eof,
}

impl TokenType {
    pub(crate) fn compound(bytes: &[u8]) -> Self {
        match bytes {
            b"!=" => TokenType::BangEqual,
            b"==" => TokenType::EqualEqual,
            b">=" => TokenType::GreaterEqual,
            b"<=" => TokenType::LessEqual,
            _ => unreachable!(),
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
            _ => unreachable!(),
        }
    }
}

// TODO: create a display adapter type that can accurately produce a display
// representation of the string, number, etc. contained in whichever overarching
// type contains itself the `TokenType` type.
impl Display for TokenType {
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
            Self::Eof => f.pad("{eof}"),
        }
    }
}
