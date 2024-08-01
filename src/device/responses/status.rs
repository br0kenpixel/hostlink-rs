use crate::protocol::Message;
use derive_more::Display;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Status {
    mode: StatusMode,
    memory: StatusMemory,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusMode {
    Program,
    Run,
    Monitor,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusMemory {
    Rom,
    Ram,
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum StatusParseError {
    #[error("Missing mode bytes")]
    MissingMode,
    #[error("Missing memory status bytes")]
    MissingMemory,
    #[error("Message contains an error")]
    UnparsableMessage,
    #[error("Unknown mode: '{0}{1}'")]
    UnknownMode(char, char),
    #[error("Unknown memory staus: '{0}{1}'")]
    UnknownMemory(char, char),
}

impl TryFrom<Message> for Status {
    type Error = StatusParseError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        if value.check_device_error().is_some() {
            return Err(Self::Error::UnparsableMessage);
        }

        let mut params_iter = value.params().iter().skip(2);
        let mode = params_iter
            .next()
            .zip(params_iter.next())
            .map(|(first, second)| StatusMode::parse(*first, *second))
            .ok_or(Self::Error::MissingMode)??;

        let memory = params_iter
            .next()
            .zip(params_iter.next())
            .map(|(first, second)| StatusMemory::parse(*first, *second))
            .ok_or(Self::Error::MissingMemory)??;

        Ok(Self { mode, memory })
    }
}

impl StatusMode {
    pub const fn parse(first: char, second: char) -> Result<Self, StatusParseError> {
        match (first, second) {
            ('0', '0') => Ok(Self::Program),
            ('0', '2') => Ok(Self::Run),
            ('0', '3') => Ok(Self::Monitor),
            _ => Err(StatusParseError::UnknownMode(first, second)),
        }
    }
}

impl StatusMemory {
    pub const fn parse(first: char, second: char) -> Result<Self, StatusParseError> {
        match (first, second) {
            ('2' | '4', '8') => Ok(Self::Rom),
            ('2' | '4', '0') => Ok(Self::Ram),
            _ => Err(StatusParseError::UnknownMemory(first, second)),
        }
    }
}
