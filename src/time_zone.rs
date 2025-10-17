use crate::{TimeDesignation, Timestamp};

/// A time zone (minus the time adjustment list)
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(C, packed)]
pub struct TimeZone {
    /// Name of the timezone
    designation: TimeDesignation,
    /// Must be 0 (potential future: 1 to disable leap seconds)
    ext: u32,
    /// Timestamp of Jan 1 00:00:00.000_000, year 0 in this timezone
    offset: Timestamp,
}
