use core::str::FromStr;

/// The mode for a given command.
#[derive(Debug, Clone, Copy)]
pub enum CmdMode {
    /// Write without expecting a response.
    WriteOnly,

    /// Write and expect a response.
    WriteWithResponse,
}

impl FromStr for CmdMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "write" => Ok(Self::WriteOnly),
            "write_with_response" => Ok(Self::WriteWithResponse),
            _ => Err("invalid mode"),
        }
    }
}
