use super::error::ValidationError;

const MAX_HALL_INTERPOLATION: u8 = 0x19;

/// Hall interpolation configuration.
///
/// This adjust how **quickly** the motor is giving boost
/// when you just start pedalling.
#[derive(Clone, Copy, Debug)]
pub struct HallInterpolation {
    interpolation: u8,
}

impl HallInterpolation {
    /// Creates a new hall interpolation configuration.
    ///
    /// The interpolation is a value between 0 and 25.
    /// Any error is returned if the value is out of range.
    pub fn new(interpolation: u8) -> Result<Self, ValidationError> {
        if interpolation > MAX_HALL_INTERPOLATION {
            return Err(ValidationError::InvalidRange {
                start: 0,
                end: MAX_HALL_INTERPOLATION,
            });
        }

        Ok(Self { interpolation })
    }

    /// Returns the hall interpolation value.
    pub fn interpolation(&self) -> u8 {
        self.interpolation
    }
}
