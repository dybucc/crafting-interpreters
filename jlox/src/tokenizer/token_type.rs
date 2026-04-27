use std::debug_assert_matches;

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
    pub(crate) fn two_chars(bytes: &[u8]) -> Self {
        match bytes {
            b"!=" => TokenType::BangEqual,
            b"==" => TokenType::EqualEqual,
            b">=" => TokenType::GreaterEqual,
            b"<=" => TokenType::LessEqual,
            _ => unreachable!(),
        }
    }

    pub(crate) fn single_char(byte: u8) -> Self {
        debug_assert_matches!(
            byte,
            b'(' | b')' | b'{' | b'}' | b',' | b'.' | b'-' | b'+' | b';' | b'*'
        );

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
