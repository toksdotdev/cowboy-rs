/// Toreque gain configuration.
#[derive(Clone, Copy, Debug)]
pub struct TorqueGain {
    pub gain: u8,
    pub unit: TorqueGainUnit,
}

/// Torque gain unit.
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
