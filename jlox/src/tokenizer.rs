mod location;

use std::borrow::Cow;

use self::location::Location;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
    EqualBang,
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

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) ty: TokenType,
    pub(crate) lex: Cow<'static, str>,
    pub(crate) lit: Option<()>,
    pub(crate) loc: Location,
}
