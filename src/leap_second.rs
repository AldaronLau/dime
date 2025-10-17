use crate::Secs;

/// A leap second
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(C, packed)]
pub(crate) struct LeapSecond {
    /// Range: 0 ~ 65_535
    year: u16,
    /// Range: (era) << 4 | 1 ~ 12
    month: u8,
    /// Either -1 or 1
    delta: i8,
}

impl LeapSecond {
    /// Get the year.
    pub(crate) fn year(&self) -> u32 {
        let era = u32::from(self.month >> 4);

        u32::from(self.year) | (era << 16)
    }

    /// Get the leap second delta (should be either -1 or 1)
    pub(crate) fn delta(&self) -> Secs {
        Secs(self.delta.into())
    }
}
