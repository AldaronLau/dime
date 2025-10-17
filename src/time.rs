/// A naïve time (unspecified timezone)
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Time {
    /// Range: 0 ~ 23
    hour: u8,
    /// Range: 1 ~ 59
    minute: u8,
    /// Range: 0 ~ 60_999 (can represent leap seconds)
    millis: u8,
}
