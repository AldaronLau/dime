use core::fmt;

/// Minutes (without accounting for leap seconds)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NaïveMinutes(pub i64);

impl fmt::Display for NaïveMinutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_str("min")
    }
}
