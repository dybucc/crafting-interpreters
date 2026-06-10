use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display as StdDisplay, Formatter},
    hash::{Hash, Hasher}
};

use super::TokenType;

#[derive_const(Clone)]
pub(crate) struct Display<'a> {
    token_type: TokenType,
    displayable: &'a dyn StdDisplay
}

impl<'a> Display<'a> {
    pub(super) fn new(token_type: TokenType, displayable: &'a impl StdDisplay) -> Self {
        Self { token_type, displayable: displayable as &dyn StdDisplay }
    }
}

/// This implementation will compare displayable objects by their token type.
const impl PartialEq for Display<'_> {
    fn eq(&self, other: &Self) -> bool { matches!(self.cmp(other), Ordering::Equal) }
}

/// This implementation will compare displayable objects by their token type.
const impl Eq for Display<'_> {}

/// This implementation will compare displayable objects by their token type.
const impl PartialOrd for Display<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.cmp(other).into() }
}

/// This implementation will compare displayable objects by their token type.
const impl Ord for Display<'_> {
    fn cmp(&self, other: &Self) -> Ordering { self.token_type.cmp(&other.token_type) }
}

/// Following the [`Ord`] implementation for [`Display`], this will hash solely
/// based on the token type.
///
/// [`Ord`]: trait@std::cmp::Ord
/// [`Display`]: struct@self::Display
impl Hash for Display<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) { self.token_type.hash(state); }
}

impl Debug for Display<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Display))
            .field("token_type", &self.token_type)
            .field("displayable", &fmt::from_fn(|f| self.displayable.fmt(f)))
            .finish()
    }
}

/// The crown jewel of the display adapter.
///
/// This will use the [`Display`] impl for the internal [`TokenType`] if it is
/// not a string, an identifier or a number. Otherwise, the displayable with
/// which `self` got constructed will be used instead.
///
/// [`Display`]: trait@std::fmt::Display
/// [``TokenType]: struct@super::TokenType
impl StdDisplay for Display<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let TokenType::Ident | TokenType::String | TokenType::Num = self.token_type {
            self.displayable.fmt(f)
        } else {
            StdDisplay::fmt(&self.token_type, f)
        }
    }
}
