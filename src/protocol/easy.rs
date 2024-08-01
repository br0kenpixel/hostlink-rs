use super::{Message, MessageKind, NodeId, ProtocolError};
use derive_more::Display;

/// A simplified representation of a command.
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum EasyCommand {
    /// Transmits a block of data, which is then repeated by the PLC.
    Test(Box<str>),
    /// Reads the operating status of the PLC.
    StatusRead,
}

impl EasyCommand {
    /// Construct a `Test` command with the given data.
    pub fn make_test<S: AsRef<str>>(data: S) -> Result<Self, ProtocolError> {
        let data = data.as_ref();

        if !data
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch.is_ascii_punctuation())
        {
            return Err(ProtocolError::InvalidTestData);
        }

        Ok(Self::Test(data.into()))
    }

    /// Construct a `StatusRead` command.
    #[must_use]
    pub const fn make_status_read() -> Self {
        Self::StatusRead
    }

    /// Perform conversion into [`Command`](Command).
    #[must_use]
    pub fn into_command(self, node: NodeId) -> Message {
        let kind = self.kind();

        match self {
            Self::Test(data) => Message::new(node, kind, data.into()),
            Self::StatusRead => Message::new_with_empty_params(node, kind),
        }
    }

    /// Get the command type.
    #[must_use]
    pub const fn kind(&self) -> MessageKind {
        match self {
            Self::Test(..) => MessageKind::Test,
            Self::StatusRead => MessageKind::StatusRead,
        }
    }
}
