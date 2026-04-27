use std::io::{self, BufReader, Cursor, Read};

use crate::{
    Error, ToError,
    tokenizer::{IoBound, Location, SyntaxError, Token},
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

        macro_rules! insert {
            ($bytes:expr, $len:expr) => {{
                out.push(Token::new($bytes, Location::new(self.line, self.col, $len)));
                self.col += 1;
            }};
        }

        // NOTE: we don't care for errors that are not syntax errors here so we just
        // propagate them up the stack.
        while self.advance(&mut buf)? {
            match buf[0] {
                b @ (b'(' | b')' | b'{' | b'}' | b',' | b'.' | b'-' | b'+' | b';' | b'*') => {
                    insert!(&[b], 1);
                }
                b @ b'/' if self.peek(b"/")? => loop {
                    self.advance(&mut buf)?;

                    if &buf == b"\n" {
                        self.line += 1;
                        self.col = 0;

                        break;
                    }
                },
                b @ (b'!' | b'=' | b'<' | b'>') if self.peek(b"=")? => {
                    let mut coalesced_symbol = vec![b];

                    self.advance(&mut buf)?;
                    coalesced_symbol.push(buf[0]);

                    insert!(&coalesced_symbol, 2);
                }
                b @ (b'!' | b'=' | b'>' | b'<') => insert!(&[b], 1),
                b'\n' => {
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
