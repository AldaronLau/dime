use crate::MonthOfYear;

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
    /// Create a new date with the day of the month.
    pub const fn with_day_of_month(
        _year: ranch::RangedU32<0, 1048575>,
        _month: MonthOfYear,
        _day_of_month: ranch::RangedU8<1, 31>,
    ) -> Self {
        todo!()
    }

    /// Create a new date with the day of the year.
    pub const fn with_day_of_year(
        _year: ranch::RangedU32<0, 1048575>,
        _month: MonthOfYear,
        _day_of_year: ranch::RangedU16<1, 366>,
    ) -> Self {
        todo!()
    }

    // FIXME: Make year signed
    /// Get the year.
    pub const fn year(&self) -> ranch::RangedU32<0, 1048575> {
        let era = (self.month >> 4) as u32;
        let Ok(year) = ranch::RangedU32::new((self.year as u32) | (era << 16))
        else {
            panic!("range logic error")
        };

        year
    }

    /// Get the day of the month.
    pub const fn day_of_month(&self) -> ranch::RangedU8<1, 31> {
        let Ok(day) = ranch::RangedU8::new((self.day << 3) >> 3) else {
            panic!("invalid date")
        };

        day
    }
}
