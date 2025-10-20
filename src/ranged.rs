use core::{error, fmt};

macro_rules! impl_ranged_fmt {
    ($type:ident, $primitive:ty, [$($Trait:ident),* $(,)?] $(,)?) => {
        $(
            impl<const MIN: $primitive, const MAX: $primitive> fmt::$Trait
            for $type<MIN, MAX>
            where
            {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    <$primitive as fmt::$Trait>::fmt(&self.get(), f)
                }
            }
        )*
    };
}

impl_ranged_fmt!(
    RangedU8,
    u8,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

/// A [`u8`] with specified minimum and maximum value.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RangedU8<const MIN: u8, const MAX: u8>(u8);

impl<const MIN: u8, const MAX: u8> RangedU8<MIN, MAX> {
    /// The size of this integer type in bits.
    pub const BITS: u32 = u8::BITS;
    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self(MAX);
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self(MIN);

    /// Try to create a new ranged integer.
    ///
    /// Returns `None` if out of bounds.
    ///
    /// ```rust
    /// # use dime::{RangedU8, Error};
    /// RangedU8::<1, 2>::new(1).unwrap();
    /// RangedU8::<1, 2>::new(2).unwrap();
    /// assert_eq!(RangedU8::<1, 2>::new(0).unwrap_err(), Error::NegOverflow(0u8));
    /// assert_eq!(RangedU8::<1, 2>::new(3).unwrap_err(), Error::PosOverflow(3u8));
    /// ```
    pub const fn new(value: u8) -> Result<Self, Error<u8>> {
        if value < MIN {
            return Err(Error::NegOverflow(value));
        }

        if value > MAX {
            return Err(Error::PosOverflow(value));
        }

        Ok(Self(value))
    }

    /// Create a new ranged integer.
    ///
    /// Won't compile if out of bounds.
    ///
    /// Compiles:
    ///
    /// ```rust
    /// # use dime::RangedU8;
    /// RangedU8::<1, 3>::new_const::<1>();
    /// RangedU8::<1, 3>::new_const::<2>();
    /// RangedU8::<1, 3>::new_const::<3>();
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// RangedU8::<1, 3>::new_const::<0>();
    /// ```
    ///
    /// ```compile_fail
    /// RangedU8::<1, 3>::new_const::<4>();
    /// ```
    pub const fn new_const<const N: u8>() -> Self {
        const {
            if N < MIN || N > MAX {
                panic!("Out of bounds");
            }
        }

        Self(N)
    }

    /// Return the contained value as a primitive type.
    ///
    /// ```rust
    /// # use dime::RangedU8;
    /// assert_eq!(42, RangedU8::<1, 100>::new_const::<42>().get());
    /// ```
    pub const fn get(self) -> u8 {
        self.0
    }

    /// Return the number of leading zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use dime::RangedU8;
    /// let n = RangedU8::<0, 255>::MAX;
    ///
    /// assert_eq!(n.leading_zeros(), 0);
    /// ```
    pub const fn leading_zeros(self) -> u32 {
        self.get().leading_zeros()
    }

    /// Return the number of trailing zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use dime::RangedU8;
    /// let n = RangedU8::<0, 255>::new_const::<0b0101000>();
    ///
    /// assert_eq!(n.trailing_zeros(), 3);
    /// ```
    pub const fn trailing_zeros(self) -> u32 {
        self.get().trailing_zeros()
    }

    /// Return the number of ones in the binary representation of `self`.
    ///
    /// ```rust
    /// # use dime::RangedU8;
    /// let a = RangedU8::<0, 255>::new_const::<0b100_0000>();
    /// let b = RangedU8::<0, 255>::new_const::<0b100_0011>();
    ///
    /// assert_eq!(a.count_ones(), 1);
    /// assert_eq!(b.count_ones(), 3);
    /// ```
    pub const fn count_ones(self) -> u32 {
        self.get().count_ones()
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        let Some(value) = self.get().checked_add(other.get()) else {
            return None;
        };

        match Self::new(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`Self::MAX`] on overflow.
    pub const fn saturating_add(self, other: Self) -> Self {
        match Self::new(self.get().saturating_add(other.get())) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_mul(self, other: Self) -> Option<Self> {
        let Some(value) = self.get().checked_mul(other.get()) else {
            return None;
        };

        match Self::new(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`Self::MAX`] on overflow.
    pub const fn saturating_mul(self, other: Self) -> Self {
        match Self::new(self.get().saturating_mul(other.get())) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_pow(self, other: u32) -> Option<Self> {
        let Some(value) = self.get().checked_pow(other) else {
            return None;
        };

        match Self::new(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns [`Self::MAX`] on overflow.
    pub const fn saturating_pow(self, other: u32) -> Self {
        match Self::new(self.get().saturating_pow(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Checked integer division.
    ///
    /// Returns [`None`] on overflow or `rhs == 0`.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        let Some(value) = self.get().checked_div(rhs.get()) else {
            return None;
        };

        match Self::new(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Saturating integer division.
    ///
    /// Returns [`Self::MIN`] on overflow.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    pub const fn saturating_div(self, rhs: Self) -> Self {
        match Self::new(self.get().saturating_div(rhs.get())) {
            Ok(value) => value,
            Err(_) => Self::MIN,
        }
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        let Some(value) = self.get().checked_sub(other.get()) else {
            return None;
        };

        match Self::new(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`Self::MIN`] on overflow.
    pub const fn saturating_sub(self, other: Self) -> Self {
        match Self::new(self.get().saturating_sub(other.get())) {
            Ok(value) => value,
            Err(_) => Self::MIN,
        }
    }

    /// Return the smallest power of two greater than or equal to self.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        let Some(value) = self.get().checked_next_power_of_two() else {
            return None;
        };

        Some(match Self::new(value) {
            Ok(value) => value,
            Err(_) => return None,
        })
    }

    /// Returns true if and only if `self == (1 << k)` for some `k`.
    pub const fn is_power_of_two(self) -> bool {
        self.get().is_power_of_two()
    }

    /// Calculate the midpoint (average) between `self` and `rhs`.
    pub const fn midpoint(self, rhs: Self) -> Self {
        let Ok(value) = Self::new(self.get().midpoint(rhs.get())) else {
            panic!("unexpected midpoint value")
        };

        value
    }
}

impl<const MIN: u8, const MAX: u8> core::str::FromStr for RangedU8<MIN, MAX> {
    type Err = ParseIntError<u8>;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let parsed = src.parse::<u8>()?;

        Self::new(parsed).map_err(From::from)
    }
}

/// Error creating ranged integer
#[derive(Eq, PartialEq, Debug)]
pub enum Error<T> {
    /// Integer is too large to store in target integer type
    PosOverflow(T),
    /// Integer is too small to store in target integer type
    NegOverflow(T),
}

impl<T> error::Error for Error<T> where T: fmt::Display + fmt::Debug {}

impl<T> fmt::Display for Error<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PosOverflow(int) => write!(
                f,
                "Integer {int} is too large to store in target integer type",
            ),
            Self::NegOverflow(int) => write!(
                f,
                "Integer {int} is too small to store in target integer type",
            ),
        }
    }
}

/// Error parsing integer
#[derive(Eq, PartialEq, Debug)]
pub enum ParseIntError<T> {
    Parsing(core::num::ParseIntError),
    PosOverflow(T),
    NegOverflow(T),
}

impl<T> error::Error for ParseIntError<T> where T: fmt::Display + fmt::Debug {}

impl<T> fmt::Display for ParseIntError<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(err) => err.fmt(f),
            Self::PosOverflow(int) => write!(
                f,
                "Integer {int} is too large to store in target integer type",
            ),
            Self::NegOverflow(int) => write!(
                f,
                "Integer {int} is too small to store in target integer type",
            ),
        }
    }
}

impl<T> From<core::num::ParseIntError> for ParseIntError<T> {
    fn from(error: core::num::ParseIntError) -> Self {
        Self::Parsing(error)
    }
}

impl<T> From<Error<T>> for ParseIntError<T> {
    fn from(error: Error<T>) -> Self {
        match error {
            Error::PosOverflow(int) => Self::PosOverflow(int),
            Error::NegOverflow(int) => Self::NegOverflow(int),
        }
    }
}
