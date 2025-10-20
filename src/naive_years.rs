use core::fmt;

/// Years (without accounting for DST, leap seconds, etc.)
///
/// # Leap year handling
///
/// During conversions, the naïve year is treated as 365.2425 naïve days.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NaïveYears(pub i64);

impl fmt::Display for NaïveYears {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_str(" years")
    }
}
