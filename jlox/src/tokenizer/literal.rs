use std::{
    borrow::Cow,
    cmp::Ordering,
    convert::Infallible,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct Lit {
    repr: LitRepr,
}

/// This implementation will return a string with the Unicode replacement
/// character if the underlying literal is a string, as Lox allows non-UTF-8
/// compliant strings.
impl Display for Lit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.repr.fmt(f)
    }
}

impl From<&[u8]> for Lit {
    fn from(value: &[u8]) -> Self {
        Self {
            repr: LitRepr::Str(Str::from(value)),
        }
    }
}

impl FromStr for Lit {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LitRepr::from_str(s).map(|repr| Self { repr })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum LitRepr {
    Str(Str),
    Num(Num),
}

impl Default for LitRepr {
    fn default() -> Self {
        Self::Str(Str::default())
    }
}

impl Display for LitRepr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(str) => str.fmt(f),
            Self::Num(num) => num.fmt(f),
        }
    }
}

impl FromStr for LitRepr {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Str::from_str(s).map(Self::Str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Num {
    repr: NumRepr,
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.repr.fmt(f)
    }
}

/// This type implements all binary relations manually because in the case of
/// being a decimal, there is no automatically-derivable implementation of a
/// strict total order.
#[derive(Debug, Clone, Copy)]
enum NumRepr {
    Decimal(f64),
    Integer(usize),
}

impl PartialOrd for NumRepr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}

impl PartialEq for NumRepr {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Ord for NumRepr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Decimal(decimal1), Self::Decimal(decimal2)) => decimal1.total_cmp(decimal2),
            (Self::Decimal(decimal), Self::Integer(integer)) => (*decimal as usize).cmp(integer),
            (Self::Integer(integer), Self::Decimal(decimal)) => integer.cmp(&(*decimal as usize)),
            (Self::Integer(integer1), Self::Integer(integer2)) => integer1.cmp(integer2),
        }
    }
}

impl Eq for NumRepr {}

impl Hash for NumRepr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Decimal(decimal) => decimal.to_bits().hash(state),
            Self::Integer(integer) => integer.hash(state),
        }
    }
}

impl Default for NumRepr {
    fn default() -> Self {
        Self::Integer(usize::default())
    }
}

impl Display for NumRepr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decimal(decimal) => decimal.fmt(f),
            Self::Integer(integer) => integer.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Str {
    repr: Cow<'static, [u8]>,
}

impl Display for Str {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        String::from_utf8_lossy_owned(self.repr.clone().into_owned()).fmt(f)
    }
}

impl From<&[u8]> for Str {
    fn from(value: &[u8]) -> Self {
        Self {
            repr: value.to_owned().into(),
        }
    }
}

impl FromStr for Str {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            repr: s.as_bytes().to_owned().into(),
        })
    }
}
