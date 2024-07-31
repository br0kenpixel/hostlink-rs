use std::num::ParseIntError;
use thiserror::Error;

/// Represents a protocol error.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid Node ID.
    #[error("Node ID must be 0..=99, got '{0}'")]
    IllegalNodeId(u8),

    /// Missing '@' symbol at start.
    #[error("Expected '@' to be the first character")]
    MissingAtSymbol,

    /// Invalid or missing Node ID.
    #[error("Expected 2 characters for Node ID")]
    MissingNodeId,

    /// Failed to parse string as integer.
    #[error("Failed to parse string as integer: {0}")]
    IntParse(#[from] ParseIntError),

    /// Missing header code (command code).
    #[error("Missing header (command) code")]
    MissingHeaderCode,

    /// Unknown or unsupported command type.
    #[error("Unknown or unsupported command: {0}")]
    UnknownCommand(String),

    /// Missing command terminator.
    #[error("Expected terminator at end of command")]
    MissingTerminator,

    /// Missing FCS checksum.
    #[error("Missing FCS checksum")]
    MissingFcs,

    /// Test command's message block contains invalid characters.
    #[error("Message block has illegal characters")]
    InvalidTestData,
}
