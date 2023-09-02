use crate::FieldError;

const MAX_FIELD_WEAKENING: u8 = 0x19;
const MAX_HALL_INTERPOLATION: u8 = 0x19;

#[derive(Clone, Copy, Debug)]
pub struct FieldWeakening {
    weakening: u8,
}

/// Hall interpolation configuration.
///
/// This adjust how **quickly** the motor is giving boost
/// when you just start pedalling.
#[derive(Clone, Copy, Debug)]
pub struct HallInterpolation {
    interpolation: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Speed {
    pub value: u8,
    pub unit: SpeedUnit,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum SpeedUnit {
    Kmh,
}

/// Toreque gain configuration.
#[derive(Clone, Copy, Debug)]
pub struct TorqueGain {
    pub gain: u8,
    pub unit: TorqueGainUnit,
}

/// Torque gain unit.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum TorqueGainUnit {
    /// Newton meters
    Nm,
}

/// Torque mode configuration.
#[derive(Clone, Copy, Debug)]
pub struct TorqueMode {
    /// Should there be a speed limit?
    ///
    /// You should set this to `false` if you want to go faster than the
    /// the maximum default assisted speed.
    pub speed_limit: bool,
}

impl FieldWeakening {
    pub fn new(weakening: u8) -> Result<Self, FieldError> {
        if weakening > MAX_FIELD_WEAKENING {
            return Err(FieldError::InvalidRange {
                start: 0,
                end: MAX_FIELD_WEAKENING,
            });
        }

        Ok(Self { weakening })
    }

    /// Returns the field weakening value.
    pub fn weakening(&self) -> u8 {
        self.weakening
    }
}

impl HallInterpolation {
    /// Creates a new hall interpolation configuration.
    ///
    /// The interpolation is a value between 0 and 25.
    /// Any error is returned if the value is out of range.
    pub fn new(interpolation: u8) -> Result<Self, FieldError> {
        if interpolation > MAX_HALL_INTERPOLATION {
            return Err(FieldError::InvalidRange {
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
