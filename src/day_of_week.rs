use ranch::RangedU8;

/// Day of the week
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum DayOfWeek {
    /// Monday
    Monday = 1,
    /// Tuesday
    Tuesday = 2,
    /// Wednesday
    Wednesday = 3,
    /// Thursday
    Thursday = 4,
    /// Friday
    Friday = 5,
    /// Saturday
    Saturday = 6,
    /// Sunday
    Sunday = 7,
}

impl From<DayOfWeek> for RangedU8<1, 7> {
    fn from(day_of_week: DayOfWeek) -> Self {
        use DayOfWeek::*;

        match day_of_week {
            Monday => RangedU8::new_const::<{ Monday as u8 }>(),
            Tuesday => RangedU8::new_const::<{ Tuesday as u8 }>(),
            Wednesday => RangedU8::new_const::<{ Wednesday as u8 }>(),
            Thursday => RangedU8::new_const::<{ Thursday as u8 }>(),
            Friday => RangedU8::new_const::<{ Friday as u8 }>(),
            Saturday => RangedU8::new_const::<{ Saturday as u8 }>(),
            Sunday => RangedU8::new_const::<{ Sunday as u8 }>(),
        }
    }
}
