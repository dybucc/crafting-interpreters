use std::borrow::Cow;

mod errors;
mod literal;
mod location;
mod scanner;
mod token_type;

pub(crate) use jlox::tokenizer::{
    errors::*, literal::Lit, location::Location, token_type::TokenType,
};

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

impl Token {
    pub(crate) fn new(bytes: &[u8], hint: Option<TokenType>, loc: Location) -> Self {
        debug_assert_ne!(bytes.len(), 0);

        match bytes.len() {
            1 => {
                let byte = bytes.first().unwrap();

                Self {
                    ty: TokenType::single_char(*byte),
                    lex: String::from(*byte as char).into(),
                    lit: None,
                    loc,
                }
            }
            2 => Self {
                ty: TokenType::two_chars(bytes),
                lex: String::from_utf8_lossy_owned(bytes.to_owned()).into(),
                lit: None,
                loc,
            },
            _ if let Some(hint @ TokenType::String) = hint => Self {
                ty: hint,
                lex: String::from_utf8_lossy_owned(bytes.to_owned()).into(),
                lit: Some(
                    String::from_utf8_lossy_owned(bytes.to_owned())
                        .parse()
                        .unwrap(),
                ),
                loc,
            },
            _ => unimplemented!(),
        }
    }
}
