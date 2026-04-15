/// This handles either writing a newline or writing some other buffer into the
/// provided writer. Both operations are followed by a `std::io::Write::flush()`
/// on the writer.
macro_rules! writef {
    ($f:tt) => {{
        match ($crate::support::writeln!($f), $f.flush()) {
            ($crate::support::Ok(_), $crate::support::Ok(_)) => $crate::support::Ok(()),
            ($crate::support::Err(err), _) => $crate::support::Err(err),
            (_, $crate::support::Err(err)) => $crate::support::Err(err),
        }
    }};
    ($f:tt, $str:tt$(, $($arg:tt),+)?) => {{
        match ($crate::support::write!($f, $str$(, $($arg),+)?), $f.flush()) {
            ($crate::support::Ok(_), $crate::support::Ok(_)) => $crate::support::Ok(()),
            ($crate::support::Err(err), _) => $crate::support::Err(err),
            (_, $crate::support::Err(err)) => $crate::support::Err(err),
        }
    }};
}
