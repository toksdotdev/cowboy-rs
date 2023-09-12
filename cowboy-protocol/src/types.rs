use crate::bounded;
use crate::error::FieldError;

const MAX_FIELD_WEAKENING: u8 = 0x64;
const MAX_HALL_INTERPOLATION: u8 = 0x19;
const ASI_FIELD_WEAKENING_MULTIPLIER: f32 = 40.96;

#[derive(Clone, Copy, Debug, Default)]
pub struct FieldWeakening {
    weakening: u8,
}

/// Hall interpolation configuration.
///
/// This adjust how **quickly** the motor is giving boost
/// when you just start pedalling.
#[derive(Clone, Copy, Debug, Default)]
pub struct HallInterpolation {
    interpolation: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Speed {
    pub value: u8,
    pub unit: SpeedUnit,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default)]
pub enum SpeedUnit {
    #[default]
    Kmh,
}

/// Toreque gain configuration.
#[derive(Clone, Copy, Debug, Default)]
pub struct TorqueGain {
    pub gain: u8,
    pub unit: TorqueGainUnit,
}

/// Torque gain unit.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default)]
pub enum TorqueGainUnit {
    /// Newton meters
    #[default]
    Nm,
}

/// Torque mode configuration.
#[derive(Clone, Copy, Debug, Default)]
pub struct TorqueMode {
    /// Should there be a speed limit?
    ///
    /// You should set this to `false` if you want to go faster than the
    /// the maximum default assisted speed.
    pub speed_limit: bool,
}

impl FieldWeakening {
    /// Set the field weakening value in percentage.
    ///
    /// The value must be between 0 and 100.
    pub fn new(weakening: u8) -> Result<Self, FieldError> {
        bounded!(weakening, MAX_FIELD_WEAKENING);
        Ok(Self { weakening })
    }

    /// Set the field weakening from the weakening value read from the bike.
    pub fn new_from_bike(weakening: u16) -> Self {
        Self {
            weakening: (weakening as f32 / ASI_FIELD_WEAKENING_MULTIPLIER).ceil() as u8,
        }
    }

    /// Returns the field weakening value in percentage.
    pub fn weakening(&self) -> u8 {
        self.weakening
    }

    /// Returns the field weakening value to be sent to the bike.
    pub fn weakening_for_bike(&self) -> u16 {
        (self.weakening as f32 * ASI_FIELD_WEAKENING_MULTIPLIER).floor() as u16
    }
}

impl HallInterpolation {
    /// Creates a new hall interpolation configuration.
    ///
    /// The interpolation is a value between 0 and 25.
    /// Any error is returned if the value is out of range.
    pub fn new(interpolation: u8) -> Result<Self, FieldError> {
        bounded!(interpolation, MAX_HALL_INTERPOLATION);
        Ok(Self { interpolation })
    }

    /// Returns the hall interpolation value.
    pub fn interpolation(&self) -> u8 {
        self.interpolation
    }
}

impl Default for Speed {
    fn default() -> Self {
        Self {
            value: 0x19,
            unit: SpeedUnit::Kmh,
        }
    }
}
