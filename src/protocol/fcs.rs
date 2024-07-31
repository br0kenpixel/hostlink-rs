use super::Error;
use std::fmt::Display;

/// FCS Checksum bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FcsBytes(u8, u8);

/// Calculates the FCS checksum from a serialized Hotlink command.
/// The input string must only contain characters which are actually needed for the checksum.
/// If the input string contains unneeded characters, they will also be accounted into the checksum.
pub fn fcs(cmd_fcs_range: &str) -> Result<FcsBytes, Error> {
    let mut fcs = 0;

    for byte in cmd_fcs_range.bytes() {
        fcs ^= byte;
    }

    let first_four = (fcs & 0b11110000) >> 4;
    let last_four = fcs & 0b00001111;

    Ok(FcsBytes(first_four, last_four))
}

impl Display for FcsBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl FcsBytes {
    /// Returns the FCS checksum bytes as a single numeric value.
    pub const fn value(self) -> u8 {
        self.1 + (self.0 * 10)
    }
}
