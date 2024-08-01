use super::{Message, MessageKind, MessageParams, NodeId, ProtocolError};
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
    /// # Example
    /// ```rust
    /// use hostlink::protocol::{EasyCommand, Message, MessageKind, MessageParams, NodeId};
    ///
    /// // Make up a zero node ID (required by the complex API)
    /// let node = NodeId::new(0).unwrap();
    ///
    /// // Make up a test message
    /// let test_message = "hello, world!";
    ///
    /// // Create a test command using the easy API:
    /// let easy_test = EasyCommand::make_test(test_message).unwrap();
    ///
    /// // Same, using the more complex API:
    /// let params = MessageParams::from(test_message);
    /// let complex_test = Message::new(node, MessageKind::Test, params);
    ///
    /// // They're the same
    /// assert_eq!(&easy_test, &complex_test);
    /// ```
    pub fn make_test<S: AsRef<str>>(data: S) -> Result<Self, ProtocolError> {
        let data = data.as_ref();

        if !data
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch.is_ascii_punctuation() || ch == ' ')
        {
            return Err(ProtocolError::InvalidTestData);
        }

        Ok(Self::Test(data.into()))
    }

    /// Construct a `StatusRead` command.
    /// # Example
    /// ```rust
    /// use hostlink::protocol::{EasyCommand, Message, MessageKind, MessageParams, NodeId};
    ///
    /// // Make up a zero node ID (required by the complex API)
    /// let node = NodeId::new(0).unwrap();
    ///
    /// // Create a status read command using the easy API:
    /// let easy_status = EasyCommand::make_status_read();
    ///
    /// // Same, using the more complex API:
    /// let complex_status = Message::new_with_empty_params(node, MessageKind::StatusRead);
    ///
    /// // They're the same
    /// assert_eq!(&easy_status, &complex_status);
    /// ```
    #[must_use]
    pub const fn make_status_read() -> Self {
        Self::StatusRead
    }

    /// Perform conversion into [`Message`](Message).
    #[must_use]
    pub fn into_message(self, node: NodeId) -> Message {
        let kind = self.kind();

        match self {
            Self::Test(data) => Message::new(node, kind, data.into()),
            Self::StatusRead => Message::new_with_empty_params(node, kind),
        }
    }

    /// Get the command's message type.
    #[must_use]
    pub const fn kind(&self) -> MessageKind {
        match self {
            Self::Test(..) => MessageKind::Test,
            Self::StatusRead => MessageKind::StatusRead,
        }
    }

    fn params(&self) -> MessageParams {
        match self {
            Self::Test(string) => string.clone().into(),
            Self::StatusRead => MessageParams::new(),
        }
    }
}

impl PartialEq<Message> for EasyCommand {
    fn eq(&self, other: &Message) -> bool {
        self.kind() == other.kind() && &self.params() == other.params()
    }
}
