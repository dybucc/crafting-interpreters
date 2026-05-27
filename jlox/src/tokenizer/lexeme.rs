use std::{
    borrow::Cow,
    convert::Infallible,
    fmt::{self, Display, Formatter},
    ops::Not,
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Lexeme {
    repr: Cow<'static, [u8]>,
}

/// This implementation will replace invalid UTF-8 ranges with
/// [`char::REPLACEMENT_CHARACTER`].
///
/// No heap allocations will be performed in the process.
///
/// [`char::REPLACEMENT_CHARACTER`]: char::REPLACEMENT_CHARACTER
impl Display for Lexeme {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.repr
            .utf8_chunks()
            .try_fold([0_u8; 4], |mut buf, chunk| {
                f.pad(chunk.valid())
                    .and_then(|()| {
                        chunk
                            .invalid()
                            .is_empty()
                            .not()
                            .then(|| f.pad(char::REPLACEMENT_CHARACTER.encode_utf8(&mut buf)))
                            .unwrap_or_else(|| Ok(()))
                    })
                    .map(|_| buf)
            })
            .map(|_| ())
    }
}

impl From<&[u8]> for Lexeme {
    fn from(value: &[u8]) -> Self {
        Self {
            repr: value.to_owned().into(),
        }
    }
}

impl FromStr for Lexeme {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            repr: s.as_bytes().to_owned().into(),
        })
    }
}
