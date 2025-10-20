/// A localized date (unspecified timezone)
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Date {
    /// Range: 0 ~ 65_535
    year: u16,
    /// Range: (era) << 4 | 1 ~ 12
    month: u8,
    /// Range: (of week: 1 ~ 7) << 5 | (of month: 1 ~ 31)
    day: u8,
}

impl Date {
    /// Get the year.
    pub const fn year(&self) -> u32 {
        let era = (self.month >> 4) as u32;

        (self.year as u32) | (era << 16)
    }
}
