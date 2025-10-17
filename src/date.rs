/// A naïve date (unspecified timezone)
#[derive(Copy, Clone, Hash, Debug)]
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
    pub fn year(&self) -> u32 {
        let era = u32::from(self.month >> 4);

        u32::from(self.year) | (era << 16)
    }
}
