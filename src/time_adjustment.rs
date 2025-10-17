use crate::DateTime;

/// A time adjustment (like daylight savings time)
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(C, packed)]
pub struct TimeAdjustment {
    /// The time to skip (skips directly to new time)
    pub old: DateTime,
    /// The time to skip to
    pub new: DateTime,
}
