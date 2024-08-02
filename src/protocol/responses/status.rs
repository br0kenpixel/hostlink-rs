use crate::protocol::Message;
use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Status {
    fals: bool,
    error: bool,
    mode: StatusMode,
    memory: StatusMemory,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusMode {
    Program,
    Run,
    Monitor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusMemory {
    /// Size of program memory in bytes (if available).
    pub size: Option<u16>,
    /// Whether the program memory is write protected.
    pub write_protection: bool,
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum StatusParseError {
    #[error("Missing mode bytes")]
    MissingMode,
    #[error("Missing memory status bytes")]
    MissingMemory,
    #[error("Message contains an error")]
    UnparsableMessage,
    #[error("Unexpected mode bits: '{0}', '{1}'")]
    UnknownMode(bool, bool),
    #[error("Unexpected memory size bits: '{0}', '{1}', '{2}'")]
    UnknownMemorySize(bool, bool, bool),
}

impl TryFrom<Message> for Status {
    type Error = StatusParseError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        if value.check_device_error().is_some() {
            return Err(Self::Error::UnparsableMessage);
        }

        // skip response code
        let mut params_iter = value.params().iter().skip(2).map(|ch| *ch as u8);

        let mode_byte = params_iter
            .next()
            .zip(params_iter.next())
            .map(|(first, second)| (first & 0b1111_0000) | (second & 0b0000_1111))
            .ok_or(Self::Error::MissingMode)?;

        let fals = (mode_byte & 0b1000_0000) > 0;
        let error = (mode_byte & 0b0001_0000) > 0;
        let mode = StatusMode::parse(mode_byte)?;

        let memory = params_iter
            .next()
            .zip(params_iter.next())
            .map(|(first, second)| (first & 0b1111_0000) | (second & 0b0000_1111))
            .ok_or(Self::Error::MissingMemory)?;
        let memory = StatusMemory::parse(memory)?;

        Ok(Self {
            fals,
            error,
            mode,
            memory,
        })
    }
}

impl StatusMode {
    pub const fn parse(byte: u8) -> Result<Self, StatusParseError> {
        let first = (byte & 0b0000_0010) > 0;
        let second = (byte & 0b0000_0001) > 0;

        match (first, second) {
            (false, false) => Ok(Self::Program),
            (true, false) => Ok(Self::Run),
            (true, true) => Ok(Self::Monitor),
            _ => Err(StatusParseError::UnknownMode(first, second)),
        }
    }
}

impl StatusMemory {
    pub const fn parse(byte: u8) -> Result<Self, StatusParseError> {
        let first = (byte & 0b0100_0000) > 0;
        let second = (byte & 0b0010_0000) > 0;
        let third = (byte & 0b0001_0000) > 0;
        let write_protection = (byte & 0b0000_1000) == 0;

        let program_area = match (first, second, third) {
            (false, false, false) => None,
            (false, false, true) => Some(4000),
            (false, true, false) => Some(8000),
            (true, false, false) => Some(7200),
            _ => return Err(StatusParseError::UnknownMemorySize(first, second, third)),
        };

        Ok(Self {
            size: program_area,
            write_protection,
        })
    }
}
