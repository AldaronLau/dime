use crate::Femtos;

/// Fraction of a second
///
/// Used for more precise timestamps
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(transparent)]
pub struct Subsec(Femtos);
