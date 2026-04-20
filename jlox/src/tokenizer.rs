mod errors;
mod location;
mod scanner;

use std::borrow::Cow;

pub(crate) use crate::tokenizer::{errors::*, location::Location};

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
pub(crate) enum Lit {
    Str(Cow<'static, str>),
    Num(Num),
}

#[derive(Debug)]
pub(crate) enum Num {
    Decimal(f64),
    Integer(usize),
}

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) ty: TokenType,
    pub(crate) lex: Cow<'static, str>,
    pub(crate) lit: Option<Lit>,
    pub(crate) loc: Location,
}
