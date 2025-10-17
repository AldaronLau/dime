use core::fmt;

/// Hours (without accounting for leap seconds)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NaïveHours(pub i64);

impl fmt::Display for NaïveHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_str("hr")
    }
}
