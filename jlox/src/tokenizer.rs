mod errors;
mod lexeme;
mod literal;
mod location;
mod scanner;
mod token_type;

pub(crate) use crate::tokenizer::{
    lexeme::Lexeme, literal::Lit, location::Location, token_type::TokenType,
};

#[derive(Debug)]
pub(crate) struct Num {
    repr: NumRepr,
}

#[derive(Debug)]
enum NumRepr {
    Decimal(f64),
    Integer(usize),
}

#[derive(Debug)]
pub(crate) struct Token {
    ty: TokenType,
    lex: Lexeme,
    lit: Option<Lit>,
    loc: Location,
}

impl Token {
    pub(crate) fn new(bytes: &[u8], hint: Option<TokenType>, loc: Location) -> Self {
        debug_assert_ne!(bytes.len(), 0);

        match bytes.len() {
            1 => {
                let byte = *bytes.first().unwrap();

                Self {
                    ty: TokenType::single(byte),
                    lex: String::from(byte as char).into(),
                    lit: None,
                    loc,
                }
            }
            2 => Self {
                ty: TokenType::compound(bytes),
                lex: String::from_utf8_lossy_owned(bytes.to_owned()).into(),
                lit: None,
                loc,
            },
            _ if let Some(ty @ TokenType::String) = hint => {
                let lex = String::from_utf8_lossy_owned(bytes.to_owned());

                Self {
                    ty,
                    lit: Some(lex.parse().unwrap()),
                    lex: lex.into(),
                    loc,
                }
            }
            _ => unimplemented!(),
        }
    }
}
