use crate::{Date, Time};

/// A naïve date and time (unspecified timezone)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(C, packed)]
pub struct DateTime {
    /// The associated date
    pub date: Date,
    /// The associated time
    pub time: Time,
}
