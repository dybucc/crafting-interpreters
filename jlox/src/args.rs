use std::borrow::Cow;

/// fetches the cli script.
///
/// the script is optional. the program will read from stdin. a script can skip
/// that.
pub(super) fn get_file() -> Option<Cow<'static, str>> { todo!() }
