use core::fmt;

/// Name of a timezone
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct TimeDesignation(u32);

impl TimeDesignation {
    /// GPS (always TAI-19s)
    pub const GPS: Self = Self(2);
    /// LORAN (always TAI-10s)
    pub const LORAN: Self = Self(1);
    /// TAI (continuous counting of SI second)
    pub const TAI: Self = Self(0);
    /// UTC (currently TAI-37s, affected by leap seconds)
    pub const UTC: Self = Self(3);

    /// Get the internal representation of this time designation
    pub const fn to_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Display for TimeDesignation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            Self::TAI => "TAI",
            Self::LORAN => "LORAN",
            Self::GPS => "GPS",
            Self::UTC => "UTC",
            _ => "Unknown",
        })
    }
}
