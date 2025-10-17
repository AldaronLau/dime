use core::fmt;

/// Name of a timezone
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct TimeDesignation(u32);

impl TimeDesignation {
    /// UTC
    pub const UTC: Self = Self(0);
}

impl fmt::Display for TimeDesignation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            Self::UTC => "UTC",
            _ => "Unknown",
        })
    }
}
