use super::error::ValidationError;

const MAX_FIELD_WEAKENING: u8 = 0x19;

#[derive(Clone, Copy, Debug)]
pub struct FieldWeakening {
    weakening: u8,
}

impl FieldWeakening {
    pub fn new(weakening: u8) -> Result<Self, ValidationError> {
        if weakening > MAX_FIELD_WEAKENING {
            return Err(ValidationError::InvalidRange {
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
