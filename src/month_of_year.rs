use ranch::RangedU8;

/// Month of the year
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum MonthOfYear {
    /// January
    January = 1,
    /// February
    February = 2,
    /// March
    March = 3,
    /// April
    April = 4,
    /// May
    May = 5,
    /// June
    June = 6,
    /// July
    July = 7,
    /// August
    August = 8,
    /// September
    September = 9,
    /// October
    October = 10,
    /// November
    November = 11,
    /// December
    December = 12,
}

impl From<MonthOfYear> for RangedU8<1, 12> {
    fn from(month_of_year: MonthOfYear) -> Self {
        use MonthOfYear::*;

        match month_of_year {
            January => RangedU8::new_const::<{ January as u8 }>(),
            February => RangedU8::new_const::<{ February as u8 }>(),
            March => RangedU8::new_const::<{ March as u8 }>(),
            April => RangedU8::new_const::<{ April as u8 }>(),
            May => RangedU8::new_const::<{ May as u8 }>(),
            June => RangedU8::new_const::<{ June as u8 }>(),
            July => RangedU8::new_const::<{ July as u8 }>(),
            August => RangedU8::new_const::<{ August as u8 }>(),
            September => RangedU8::new_const::<{ September as u8 }>(),
            October => RangedU8::new_const::<{ October as u8 }>(),
            November => RangedU8::new_const::<{ November as u8 }>(),
            December => RangedU8::new_const::<{ December as u8 }>(),
        }
    }
}
