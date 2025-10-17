use core::fmt;

/// Days (without accounting for timezone-specific time adjustments)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct NaïveDays(pub i64);

impl fmt::Display for NaïveDays {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_str(" days")
    }
}
