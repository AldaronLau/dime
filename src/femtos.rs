use core::fmt;

/// Femtoseconds (10⁻¹⁵s)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct Femtos(pub i64);

impl fmt::Display for Femtos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        f.write_str("fs")
    }
}
