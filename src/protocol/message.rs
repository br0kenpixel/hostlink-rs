use super::ProtocolError;
use crate::device::DeviceError;
use derive_more::Display;
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// A Hotlink command type.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageKind {
    #[display(fmt = "IR/SR AREA READ")]
    IrSrAreaRead,
    #[display(fmt = "LR AREA READ")]
    LrAreaRead,
    #[display(fmt = "HR AREA READ")]
    HrAreaRead,
    #[display(fmt = "PV READ")]
    PvRead,
    #[display(fmt = "TC STATUS READ")]
    TcStatusRead,
    #[display(fmt = "DM AREA READ")]
    DmAreaRead,
    #[display(fmt = "AR AREA READ")]
    ArAreaRead,
    #[display(fmt = "IR/SR AREA WRITE")]
    IrSrAreaWrite,
    #[display(fmt = "LR AREA WRITE")]
    LrAreaWrite,
    #[display(fmt = "HR AREA WRITE")]
    HrAreaWrite,
    #[display(fmt = "PV WRITE")]
    PvWrite,
    #[display(fmt = "TC STATUS WRITE")]
    TcStatusWrite,
    #[display(fmt = "DM AREA WRITE")]
    DmAreaWrite,
    #[display(fmt = "AR AREA WRITE")]
    ArAreaWrite,
    #[display(fmt = "SV READ 1")]
    SvRead1,
    #[display(fmt = "SV READ 2")]
    SvRead2,
    #[display(fmt = "SV READ 3")]
    SvRead3,
    #[display(fmt = "SV CHANGE 1")]
    SvChange1,
    #[display(fmt = "SV CHANGE 2")]
    SvChange2,
    #[display(fmt = "SV CHANGE 3")]
    SvChange3,
    #[display(fmt = "STATUS READ")]
    StatusRead,
    #[display(fmt = "STATUS WRITE")]
    StatusWrite,
    #[display(fmt = "ERROR READ")]
    ErrorRead,
    #[display(fmt = "FORCED SET")]
    ForcedSet,
    #[display(fmt = "FORCED RESET")]
    ForcedReset,
    #[display(fmt = "MULTIPLE FORCED SET/RESET")]
    MultipleForcedSetReset,
    #[display(fmt = "FORCED SET/RESET CANCEL")]
    ForcedSetResetCancel,
    #[display(fmt = "PC MODEL READ")]
    PcModelRead,
    #[display(fmt = "TEST")]
    Test,
    #[display(fmt = "PROGRAM READ")]
    ProgramRead,
    #[display(fmt = "PROGRAM WRITE")]
    ProgramWrite,
    #[display(fmt = "COMPOUND COMMAND")]
    CompoundCommand,
}

/// Stores a command's parameters as ASCII values.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageParams(Vec<char>);

/// Represents a Node ID - i.e. a number between 0 and 99.
#[derive(Debug, Display, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display(fmt = "{:02}", "self.0")]
pub struct NodeId(u8);

/// A complete Hotlink command.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
    /// Node ID
    node: NodeId,
    /// Command type
    kind: MessageKind,
    /// Arguments
    params: MessageParams,
}

impl Message {
    /// Creates a new command from the specified node ID and arguments.
    #[must_use]
    pub const fn new(node: NodeId, kind: MessageKind, params: MessageParams) -> Self {
        Self { node, kind, params }
    }

    /// Creates a new command with no arguments from the specified node ID.
    #[must_use]
    pub const fn new_with_empty_params(node: NodeId, kind: MessageKind) -> Self {
        Self {
            node,
            kind,
            params: MessageParams::new(),
        }
    }

    pub fn as_device_error(self) -> Result<DeviceError, ProtocolError> {
        let string: String = self.params.clone().iter().collect();

        DeviceError::try_from(string.as_str())
    }

    /// Serializes the command into a string that can be sent to a PLC.
    pub fn serialize(self) -> Result<Box<str>, ProtocolError> {
        let mut buffer = String::with_capacity(10);

        // begin
        buffer.push('@');

        // node number
        buffer.push_str(&self.node.to_string());

        // header code
        buffer.push_str(self.kind.code());

        // params
        self.params.iter().for_each(|ch| buffer.push(*ch));

        // FCS checksum
        let fcs = super::fcs::fcs(&buffer)?;
        buffer.push_str(&fcs.to_string());

        // terminator
        buffer.push_str("*\r");

        Ok(buffer.into_boxed_str())
    }

    pub fn set_node_id(&mut self, node: NodeId) {
        self.node = node;
    }

    pub fn parse(cmd: &str) -> Result<Self, ProtocolError> {
        let mut cmd_iter = cmd.chars();

        if cmd_iter.next() != Some('@') {
            return Err(ProtocolError::MissingAtSymbol);
        }

        let node_id: u8 = cmd_iter
            .next()
            .zip(cmd_iter.next())
            .map(|(first, last)| format!("{first}{last}"))
            .ok_or(ProtocolError::MissingNodeId)?
            .parse()?;
        let node_id = NodeId::new(node_id)?;
        dbg!(node_id);

        let header_code_chars = cmd_iter
            .next()
            .zip(cmd_iter.next())
            .map(|(first, last)| format!("{first}{last}"))
            .ok_or(ProtocolError::MissingHeaderCode)?;

        let command_kind = MessageKind::from_str(&header_code_chars)?;
        let mut rest: String = cmd_iter.collect();

        if rest.pop().zip(rest.pop()) != Some(('\r', '*')) {
            return Err(ProtocolError::MissingTerminator);
        }

        // we won't store the FCS, but only chceck if it's there
        rest.pop()
            .zip(rest.pop())
            .map(|(last, first)| format!("{first}{last}"))
            .ok_or(ProtocolError::MissingFcs)?;

        let params: Vec<char> = rest.chars().collect();

        Ok(Self::new(node_id, command_kind, params.into()))
    }
}

impl MessageParams {
    /// Creates an empty argument set.
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<Box<str>> for MessageParams {
    fn from(value: Box<str>) -> Self {
        Self(value.chars().collect())
    }
}

impl From<&str> for MessageParams {
    fn from(value: &str) -> Self {
        Self(value.chars().collect())
    }
}

impl From<Vec<char>> for MessageParams {
    fn from(value: Vec<char>) -> Self {
        Self(value)
    }
}

impl Deref for MessageParams {
    type Target = [char];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for NodeId {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MessageParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MessageKind {
    /// Returns the command code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::IrSrAreaRead => "RR",
            Self::LrAreaRead => "RL",
            Self::HrAreaRead => "RH",
            Self::PvRead => "RC",
            Self::TcStatusRead => "RG",
            Self::DmAreaRead => "RD",
            Self::ArAreaRead => "RJ",
            Self::IrSrAreaWrite => "WR",
            Self::LrAreaWrite => "WL",
            Self::HrAreaWrite => "WH",
            Self::PvWrite => "WC",
            Self::TcStatusWrite => "WG",
            Self::DmAreaWrite => "WD",
            Self::ArAreaWrite => "WJ",
            Self::SvRead1 => "R#",
            Self::SvRead2 => "R$",
            Self::SvRead3 => "R%",
            Self::SvChange1 => "W#",
            Self::SvChange2 => "W$",
            Self::SvChange3 => "W%",
            Self::StatusRead => "MS",
            Self::StatusWrite => "SC",
            Self::ErrorRead => "MF",
            Self::ForcedSet => "KS",
            Self::ForcedReset => "KR",
            Self::MultipleForcedSetReset => "FK",
            Self::ForcedSetResetCancel => "KC",
            Self::PcModelRead => "MM",
            Self::Test => "TS",
            Self::ProgramRead => "RP",
            Self::ProgramWrite => "WP",
            Self::CompoundCommand => "QQ",
        }
    }
}

impl FromStr for MessageKind {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RR" => Ok(Self::IrSrAreaRead),
            "RL" => Ok(Self::LrAreaRead),
            "RH" => Ok(Self::HrAreaRead),
            "RC" => Ok(Self::PvRead),
            "RG" => Ok(Self::TcStatusRead),
            "RD" => Ok(Self::DmAreaRead),
            "RJ" => Ok(Self::ArAreaRead),
            "WR" => Ok(Self::IrSrAreaWrite),
            "WL" => Ok(Self::LrAreaWrite),
            "WH" => Ok(Self::HrAreaWrite),
            "WC" => Ok(Self::PvWrite),
            "WG" => Ok(Self::TcStatusWrite),
            "WD" => Ok(Self::DmAreaWrite),
            "WJ" => Ok(Self::ArAreaWrite),
            "R#" => Ok(Self::SvRead1),
            "R$" => Ok(Self::SvRead2),
            "R%" => Ok(Self::SvRead3),
            "W#" => Ok(Self::SvChange1),
            "W$" => Ok(Self::SvChange2),
            "W%" => Ok(Self::SvChange3),
            "MS" => Ok(Self::StatusRead),
            "SC" => Ok(Self::StatusWrite),
            "MF" => Ok(Self::ErrorRead),
            "KS" => Ok(Self::ForcedSet),
            "KR" => Ok(Self::ForcedReset),
            "FK" => Ok(Self::MultipleForcedSetReset),
            "KC" => Ok(Self::ForcedSetResetCancel),
            "MM" => Ok(Self::PcModelRead),
            "TS" => Ok(Self::Test),
            "RP" => Ok(Self::ProgramRead),
            "WP" => Ok(Self::ProgramWrite),
            "QQ" => Ok(Self::CompoundCommand),
            _ => Err(ProtocolError::UnknownCommand(s.into())),
        }
    }
}

impl NodeId {
    /// Safely constructs a Node ID from the specified value.
    /// If the value is higher than 99, an error will be returned.
    pub fn new(value: u8) -> Result<Self, ProtocolError> {
        if !(0..=99).contains(&value) {
            return Err(ProtocolError::IllegalNodeId(value));
        }

        Ok(Self(value))
    }

    /// Constructs a Node ID **without** verifying the actual value.
    /// # Safety
    /// This function is only safe if the specified value is at most 99.
    /// Values above 99 will cause undefined behaviour.
    #[must_use]
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self(value)
    }
}
