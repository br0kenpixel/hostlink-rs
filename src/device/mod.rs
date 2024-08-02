mod error;

use crate::protocol::responses::status::Status;
use crate::protocol::{Message, MessageKind, MessageParams, NodeId, ProtocolError};
pub use error::{DeviceError, Error};
pub use serialport::{DataBits, FlowControl, SerialPort, SerialPortBuilder, StopBits};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    time::Duration,
};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3);

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
        let params: MessageParams = "!rust!".into();
        let command = Message::new(self.node_id, MessageKind::Test, params);

        self._send_commnad(command.clone())?;
        let response = self._await_response()?;

        if response == command {
            return Ok(());
        }

        let dev_err = response.as_device_error()?;
        dev_err.to_result()?;

        unreachable!()
    }

    pub fn status(&mut self) -> Result<Status, Error> {
        let response = self._send_command_and_await_response(
            Message::new_with_empty_params(self.node_id, MessageKind::StatusRead),
            true,
        )?;

        let status = Status::try_from(response).map_err(ProtocolError::StatusParse)?;

        Ok(status)
    }

    fn _send_command_and_await_response(
        &mut self,
        cmd: Message,
        error_check: bool,
    ) -> Result<Message, Error> {
        self._send_commnad(cmd)?;

        if error_check {
            self._await_response_and_err_check()
        } else {
            self._await_response()
        }
    }

    fn _send_commnad(&mut self, mut cmd: Message) -> Result<(), Error> {
        cmd.set_node_id(self.node_id);
        self.writer.write_all(cmd.serialize()?.as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }

    fn _await_response_and_err_check(&mut self) -> Result<Message, Error> {
        let msg = self._await_response()?;

        if let Some(error) = msg.check_device_error() {
            return Err(Error::Device(error));
        }

        Ok(msg)
    }

    fn _await_response(&mut self) -> Result<Message, Error> {
        let mut buffer = Vec::new();
        self.reader.read_until(b'\r', &mut buffer)?;

        let string = std::str::from_utf8(&buffer)?;
        let msg = Message::parse(string)?;

        Ok(msg)
    }
}
