mod error;

pub use error::Error;
pub use serialport::{DataBits, FlowControl, SerialPort, SerialPortBuilder, StopBits};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    time::Duration,
};

use crate::protocol::{Command, CommandKind, CommandParams, NodeId};

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3);
pub const RCV_BUFSIZE: usize = 128;

#[derive(Debug)]
pub struct PlcDevice {
    reader: BufReader<Box<dyn SerialPort>>,
    writer: BufWriter<Box<dyn SerialPort>>,
    node_id: NodeId,
}

impl PlcDevice {
    pub fn connect(
        mut port: Box<dyn SerialPort>,
        node_id: NodeId,
        timeout: Option<Duration>,
    ) -> Result<Self, Error> {
        port.set_timeout(timeout.unwrap_or(DEFAULT_TIMEOUT))?;

        let reader = port;
        let writer = reader.try_clone()?;

        Ok(Self {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
            node_id,
        })
    }

    pub fn connect_with_builder(
        builder: SerialPortBuilder,
        node_id: NodeId,
        timeout: Option<Duration>,
    ) -> Result<Self, Error> {
        Self::connect(builder.open()?, node_id, timeout)
    }

    pub fn test(&mut self) -> Result<(), Error> {
        let params: CommandParams = "!rust!".into();
        let command = Command::new(self.node_id, CommandKind::Test, params);

        self._send_commnad(command.clone())?;
        let response = self._await_response()?;
        let response = Command::parse(&response)?;

        if response == command {
            return Ok(());
        }

        unimplemented!()
    }

    fn _send_commnad(&mut self, mut cmd: Command) -> Result<(), Error> {
        dbg!(&cmd);
        cmd.set_node_id(self.node_id);
        self.writer.write_all(cmd.serialize()?.as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }

    fn _await_response(&mut self) -> Result<Box<str>, Error> {
        let mut buffer = Vec::new();
        self.reader.read_until(b'\r', &mut buffer)?;

        let string = std::str::from_utf8(&buffer)?;

        Ok(string.into())
    }
}
