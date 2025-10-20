use crate::{Date, Time};

/// A localized date and time (unspecified timezone)
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct DateTime {
    /// The associated date
    pub date: Date,
    /// The associated time
    pub time: Time,
}
