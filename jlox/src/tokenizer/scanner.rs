use std::{
    io::{self, BufReader, Cursor, Read},
    ops::Not,
};

use crate::{
    Error, ToError,
    tokenizer::{IoBound, Location, SyntaxError, Token, TokenType},
};

#[derive(Debug)]
pub(crate) struct Scanner<'a> {
    pub(crate) buf: BufReader<Cursor<&'a [u8]>>,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self {
            buf: BufReader::new(Cursor::new(buf)),
            col: 0,
            line: 0,
        }
    }
}

impl Scanner<'_> {
    pub(crate) fn advance(&mut self, buf: &mut [u8; 1]) -> Result<bool, Error> {
        let Self {
            buf: source, line, ..
        } = self;

        if let Err(err) = source.read_exact(buf) {
            if matches!(err.kind(), io::ErrorKind::UnexpectedEof) {
                return Ok(true);
            }

            return Err(IoBound {
                inner: err.into(),
                line: *line,
            }
            .convert(None));
        }

        Ok(false)
    }

    pub(crate) fn peek(&mut self, bytes: &[u8]) -> Result<bool, Error> {
        let Self { buf, line, .. } = self;

        buf.peek(1)
            .map(|peeker| bytes == peeker)
            .map_err(Into::into)
            .map_err(|inner| IoBound { inner, line: *line }.convert(None))
    }

    #[expect(unused, reason = "WIP.")]
    pub(crate) fn scan(&mut self) -> Result<Vec<Token>, Error> {
        let mut buf = [0; 1];
        let mut out = Vec::new();
        let mut errors = Vec::new();

        // NOTE: this buffer we use when parsing multi-byte tokens, like those of the
        // two-byte variants of the bang symbol, or just strings. Because this only gets
        // used alongside tokens of length greater than 1, we construct it with at least
        // two elements.
        let mut running_buf = Vec::with_capacity(2);

        macro_rules! insert_token {
            ($bytes:expr, $len:expr) => {{
                out.push(Token::new(
                    $bytes,
                    None,
                    Location::new(self.line, self.col, $len),
                ));

                self.col += 1;
            }};
            ($bytes:expr, $hint:expr, $len:expr) => {{
                out.push(Token::new(
                    $bytes,
                    $hint.into(),
                    Location::new(self.line, self.col, $len),
                ));

                self.col += 1;
            }};
        }

        // NOTE: we don't care for errors that are not syntax errors here so we just
        // propagate them up the stack.
        while self.advance(&mut buf)? {
            match buf[0] {
                b @ b'"' => {
                    running_buf.clear();

                    loop {
                        if self.advance(&mut buf)?.not() {
                            errors.push(SyntaxError {
                                span: Location::new(self.line, self.col, running_buf.len()),
                            });

                            break;
                        }

                        if b == b'"' {
                            break;
                        }

                        running_buf.push(buf[0]);
                    }

                    insert_token!(&running_buf, TokenType::String, running_buf.len());
                }
                b @ (b'(' | b')' | b'{' | b'}' | b',' | b'.' | b'-' | b'+' | b';' | b'*') => {
                    insert_token!(&[b], 1);
                }
                b @ b'/' if self.peek(b"/")? => loop {
                    if self.advance(&mut buf)?.not() {
                        break;
                    }

                    if &buf == b"\n" {
                        self.line += 1;
                        self.col = 0;

                        break;
                    }
                },
                b @ (b'!' | b'=' | b'<' | b'>') if self.peek(b"=")? => {
                    running_buf.clear();
                    running_buf.push(b);

                    self.advance(&mut buf)?;
                    running_buf.push(buf[0]);

                    insert_token!(&running_buf, 2);
                }
                b @ (b'!' | b'=' | b'>' | b'<') => insert_token!(&[b], 1),
                b'\n' => {
                    // NOTE: at the end of each line we parse, errors (within the same line) that
                    // happen within one byte offset of each other are merge into a single error.
                    while errors.len() > 1 {
                        let last_err: SyntaxError = errors.pop().unwrap();
                        let before_last = errors.last_mut().unwrap();

                        if before_last.same_line(&last_err) {
                            if before_last.akin_spans(&last_err) {
                                before_last.merge(&last_err);
                            }
                        } else {
                            errors.push(last_err);

                            break;
                        }
                    }

                    self.line += 1;
                    self.col = 0;
                }
                // NOTE: we require special treatment of line feeds to update internal counters, so
                // if there's an ascii whitespace match beyond that, we can be sure it's not a line
                // feed and can thus be safely ignored.
                b if b.is_ascii_whitespace() => (),
                _ => errors.push(SyntaxError::new(Location {
                    line: self.line,
                    col: self.col,
                    len: 1,
                })),
            }
        }

        Ok(out)
    }
}
