use std::{
    io::{self, BufReader, Cursor, Read},
    ops::Not,
};

use crate::{
    Error, ErrorTrace, ToError,
    tokenizer::{
        IoBound, Location, MalformedNumber, SyntaxError, Token, TokenType, UnexpectedEof,
        scanner::peeker_pattern::PeekerPattern,
    },
};

mod peeker_pattern;

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
                return Ok(false);
            }

            return Err(IoBound {
                inner: err.into(),
                line: *line,
            }
            .convert(None));
        }

        Ok(true)
    }

    // TODO: handle reaching EOF, as that should be reported with a non-error
    // variant in the same fashion as with `advance()`.
    pub(crate) fn peek(&mut self, mut pat: impl PeekerPattern) -> Result<bool, Error> {
        let Self { buf, line, .. } = self;

        buf.peek(1)
            .map(pat.eval())
            .map_err(Into::into)
            .map_err(|inner| IoBound { inner, line: *line }.convert(None))
    }

    #[expect(unused, reason = "WIP.")]
    pub(crate) fn scan(&mut self) -> Result<Vec<Token>, Error> {
        let mut buf = [0; 1];
        let mut out = Vec::new();
        let mut errors = Vec::new();

        // NOTE: this buffer we use when parsing multi-byte tokens, which is to say all
        // tokens other than single byte operators. Because this only gets used
        // alongside tokens of length greater than 1, we construct it with at least two
        // elements.
        let mut running_buf = Vec::with_capacity(2);

        macro_rules! new_token {
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
                b if b.is_ascii_digit() => {
                    running_buf.clear();
                    running_buf.push(b);

                    let mut in_float = false;

                    loop {
                        if self.peek(b".")? {
                            if in_float.not() {
                                in_float = true;
                            } else {
                                errors.push(SyntaxError::new(
                                    Location::new(self.line, self.col, running_buf.len()),
                                    Box::new(MalformedNumber) as Box<dyn ErrorTrace>,
                                ));

                                break;
                            }
                        }

                        if self.peek(|bytes: &[u8]| bytes.first().unwrap().is_ascii_whitespace())? {
                        }
                    }
                }
                b @ b'"' => {
                    running_buf.clear();

                    loop {
                        if self.advance(&mut buf)?.not() {
                            errors.push(SyntaxError::new(
                                Location::new(self.line, self.col, running_buf.len()),
                                Box::new(UnexpectedEof) as Box<dyn ErrorTrace>,
                            ));

                            break;
                        }

                        if b == b'"' {
                            break;
                        }

                        running_buf.push(buf[0]);
                    }

                    // NOTE: the hint we provide to the token generator makes it so that the string
                    // need not be utf-8, as whatever bytes are not will be filled in with the
                    // replacement character.
                    new_token!(&running_buf, TokenType::String, running_buf.len());
                }
                b @ (b'(' | b')' | b'{' | b'}' | b',' | b'.' | b'-' | b'+' | b';' | b'*') => {
                    new_token!(&[b], 1);
                }
                b @ b'/' if self.peek(b"/")? => loop {
                    // NOTE: supporting single-line comments does not require checking if EOF has
                    // been hit, as reaching it does not cause parsing issues with respecto the rest
                    // of the grammar.
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

                    new_token!(&running_buf, 2);
                }
                b @ (b'!' | b'=' | b'>' | b'<') => new_token!(&[b], 1),
                b'\n' => {
                    // NOTE: at the end of each line we parse, errors (within the same line) that
                    // happen within one byte offset of each other are merged into a single error.
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
                _ => errors.push(SyntaxError::new_generic(Location {
                    line: self.line,
                    col: self.col,
                    len: 1,
                })),
            }
        }

        Ok(out)
    }
}
