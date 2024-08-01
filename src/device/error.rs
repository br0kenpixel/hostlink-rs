use std::{io, str::Utf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serial: {0}")]
    Serial(#[from] serialport::Error),

    #[error("IO: {0}")]
    Io(#[from] io::Error),

    #[error("Protocol: {0}")]
    Protocol(#[from] crate::protocol::ProtocolError),

    #[error("Failed to parse a UTF-8 string: {0}")]
    StringConversion(#[from] Utf8Error),

    #[error("Device reported error: {0}")]
    Device(#[from] DeviceError),
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeviceError {
    #[error("No error")]
    None,
    #[error("Not executable in RUN mode")]
    NotExecutableInRunMode,
    #[error("Not executable in MONITOR mode")]
    NotExecutableInMonitorMode,
    #[error("Not executable with PROM mounted")]
    NotExecutableWithPromMounted,
    #[error("Address over (data overflow)")]
    AddressOver,
    #[error("1/0 REGISTER capacity exceeded (no registration made), I/0 READ unexecutable.")]
    IoRegisterCapacityExceeded,
    #[error("Not executable in PROGRAM mode")]
    NotExecutableInProgramMode,
    #[error("Parity error")]
    ParityError,
    #[error("Framing error")]
    FramingError,
    #[error("Overrun")]
    Overrun,
    #[error("FCS error")]
    FCSError,
    #[error("Format error (parameter length error)")]
    FormatError,
    #[error("Entry number data error (parameter error, data code error, data length error)")]
    EntryNumberData,
    #[error("Instruction not found")]
    InstructionNotFound,
    #[error("Frame length error")]
    FrameLengthError,
    #[error(
        "Not executable (due to unexecutable error clear, non-registration of I/O table, etc.)"
    )]
    NotExecutable,
    #[error("Aborted due to parity error in transmit data")]
    BadParity,
    #[error("Aborted due to framing error in transmit data")]
    BadFraming,
    #[error("Aborted due to overrun in transmit data")]
    TransmitDataOverrun,
    #[error("Aborted due to format error in transmit data")]
    Format,
    #[error("Aborted due to entry number data error in transmit data")]
    IllegalEntryNumber,
    #[error("Aborted due to frame length error in transmit data")]
    IllegalFrameLength,
}

impl DeviceError {
    pub const fn to_result(self) -> Result<(), DeviceError> {
        if matches!(self, Self::None) {
            Ok(())
        } else {
            Err(self)
        }
    }

    pub const fn is_ok(self) -> bool {
        self.to_result().is_ok()
    }
}

impl TryFrom<(char, char)> for DeviceError {
    type Error = crate::protocol::ProtocolError;

    fn try_from(value: (char, char)) -> Result<Self, Self::Error> {
        match value {
            ('0', '0') => Ok(Self::None),
            ('0', '1') => Ok(Self::NotExecutableInRunMode),
            ('0', '2') => Ok(Self::NotExecutableInMonitorMode),
            ('0', '3') => Ok(Self::NotExecutableWithPromMounted),
            ('0', '4') => Ok(Self::AddressOver),
            ('0', '9') => Ok(Self::IoRegisterCapacityExceeded),
            ('0', 'B') => Ok(Self::NotExecutableInProgramMode),
            ('1', '0') => Ok(Self::ParityError),
            ('1', '1') => Ok(Self::FramingError),
            ('1', '2') => Ok(Self::Overrun),
            ('1', '3') => Ok(Self::FCSError),
            ('1', '4') => Ok(Self::FormatError),
            ('1', '5') => Ok(Self::EntryNumberData),
            ('1', '6') => Ok(Self::InstructionNotFound),
            ('1', '8') => Ok(Self::FrameLengthError),
            ('1', '9') => Ok(Self::NotExecutable),
            ('A', '0') => Ok(Self::BadParity),
            ('A', '1') => Ok(Self::BadFraming),
            ('A', '2') => Ok(Self::TransmitDataOverrun),
            ('A', '4') => Ok(Self::Format),
            ('A', '5') => Ok(Self::IllegalEntryNumber),
            ('A', '8') => Ok(Self::IllegalFrameLength),
            _ => Err(Self::Error::UnknownErrorCode(value.0, value.1)),
        }
    }
}

impl TryFrom<&str> for DeviceError {
    type Error = crate::protocol::ProtocolError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(Self::Error::ErrorCodeBadLength);
        }

        let mut chars = value.chars();
        let first = unsafe { chars.next().unwrap_unchecked() };
        let second = unsafe { chars.next().unwrap_unchecked() };

        Self::try_from((first, second))
    }
}
