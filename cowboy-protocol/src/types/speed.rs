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
