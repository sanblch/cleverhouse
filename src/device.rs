use crate::devices::smartsocket::SmartSocket;
use crate::devices::thermometer::Thermometer;

pub enum DeviceType {
    SmartSocket(SmartSocket),
    Thermometer(Thermometer),
}

pub trait Device {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}
