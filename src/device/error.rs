use std::{io, str::Utf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serial: {0}")]
    Serial(#[from] serialport::Error),

    #[error("IO: {0}")]
    Io(#[from] io::Error),

    #[error("Protocol: {0}")]
    Protocol(#[from] crate::protocol::Error),

    #[error("Failed to parse a UTF-8 string: {0}")]
    StringConversion(#[from] Utf8Error),

    #[error("Device reported error: {0}")]
    Device(#[from] DeviceError),
}

#[derive(Debug, Error)]
pub enum DeviceError {}
