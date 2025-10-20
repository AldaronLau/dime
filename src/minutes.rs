use core::fmt;

/// Minutes
///
/// This is observed time, not clock time (doesn't account for leap seconds,
/// always 60 seconds).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct Minutes(pub i64);

impl fmt::Display for Minutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_str("min")
    }
}
