use std::io::{Cursor, Read, Seek};

use crate::{
    Error, ToError,
    tokenizer::{InvalidUtf8, IoBound, Location, Token, TokenType, UnexpectedEof},
};

#[derive(Debug)]
pub(crate) struct Scanner<'a> {
    pub(crate) cursor: Cursor<&'a [u8]>,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(buf),
            col: 0,
            line: 0,
        }
    }

    fn scan(&mut self) -> Result<Vec<Token>, Error> {
        let Self {
            cursor,
            line: line_num,
            col: col_num,
        } = self;
        let mut buf = [0; 1];

        let mut out = Vec::with_capacity(cursor.stream_len().unwrap() as usize);

        loop {
            let mut line = {
                let Some((line, _)) = cursor.get_ref().split_once(|byte| *byte == b'\n') else {
                    break;
                };

                Cursor::new(line)
            };
            line.read_exact(&mut buf).map_err(|inner| {
                IoBound {
                    inner: inner.into(),
                    line: *line_num,
                }
                .convert(None)
            })?;

            match buf.as_slice() {
                b"\n" => todo!(),
                b"(" | b")" | b"{" | b"}" | b"," | b"." | b"-" | b"+" | b";" | b"*" => {
                    let token = Token {
                        ty: TokenType::LeftParen,
                        lex: str::from_utf8(buf.clone())
                            .to_owned()
                            .map_err(|_| {
                                InvalidUtf8 {
                                    line: *line_num,
                                    col: *col_num,
                                }
                                .convert(None)
                            })?
                            .into(),
                        lit: None,
                        loc: Location::new(*line_num, *col_num, 1),
                    };

                    *col_num += 1;
                }
                b"/" => todo!(),
                b"!" => todo!(),
                b"=" => todo!(),
                b">" => todo!(),
                b"<" => todo!(),
                _ => return Err(UnexpectedEof.convert(None)),
            }
        }

        Ok(out)
    }
}
