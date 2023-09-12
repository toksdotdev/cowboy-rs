#[derive(Clone, Copy, Debug)]
pub enum FieldError {
    InvalidRange { start: u8, end: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdError {
    /// Cannot generate instruction for strictly read command.
    StrictlyRead,
}
