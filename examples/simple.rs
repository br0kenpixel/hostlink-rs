use hostlink::{device::PlcDevice, protocol::NodeId};
use serialport::{DataBits, FlowControl, StopBits};

fn main() {
    let mut device = PlcDevice::connect_with_builder(
        serialport::new("/dev/cu.usbserial-14130", 9600)
            .data_bits(DataBits::Seven)
            .flow_control(FlowControl::None)
            .parity(serialport::Parity::Even)
            .stop_bits(StopBits::Two),
        NodeId::new(0).unwrap(),
        None,
    )
    .unwrap();

    println!("Connected");
    device.test().unwrap();
    println!("OK");
}
