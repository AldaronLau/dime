use core::fmt;

/// Picoseconds (10⁻¹²s)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct Picos(pub i64);

impl fmt::Display for Picos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_str("ps")
    }
}
