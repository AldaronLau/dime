use core::fmt;

/// Weeks (without accounting for timezone-specific time adjustments)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NaïveWeeks(pub i64);

impl fmt::Display for NaïveWeeks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_str(" weeks")
    }
}
