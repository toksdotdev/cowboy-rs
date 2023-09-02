#[derive(Clone, Copy, Debug)]
pub enum ValidationError {
    InvalidRange { start: u8, end: u8 },
}
