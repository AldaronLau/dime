use core::fmt;

/// Months (without accounting for DST, leap seconds, etc.)
///
/// # Leap year handling
///
/// During conversions, the naïve month is treated as 30.436875 naïve days.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NaïveMonths(pub i64);

impl fmt::Display for NaïveMonths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_str(" years")
    }
}
