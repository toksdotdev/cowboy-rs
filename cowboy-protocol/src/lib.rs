#![no_std]

mod cmd;
pub mod types;
mod utils;

pub use self::cmd::*;

#[derive(Clone, Copy, Debug)]
pub enum FieldError {
    InvalidRange { start: u8, end: u8 },
}
