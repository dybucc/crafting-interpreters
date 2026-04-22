mod errors;
mod location;
mod scanner;
mod token_type;

use std::{borrow::Cow, debug_assert_matches};

pub(crate) use crate::tokenizer::{errors::*, location::Location, token_type::TokenType};

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

impl Token {
    pub(crate) fn new(bytes: &[u8], loc: Location) -> Self {
        debug_assert_ne!(bytes.len(), 0);

        if bytes.len() == 1 {
            let byte = bytes.first().unwrap();

            Self {
                ty: TokenType::single_char(*byte),
                lex: String::from(*byte as char).into(),
                lit: None,
                loc,
            }
        } else {
            todo!()
        }
    }
}
