use crate::devices::smartsocket::SmartSocket;
use crate::devices::thermometer::Thermometer;

#[derive(Debug, PartialEq)]
pub enum DeviceType {
    SmartSocket(SmartSocket),
    Thermometer(Thermometer),
}

/// Here is a use example
/// ```
/// use cleverhouse_by_akrutikov::device::Device;
/// pub struct SomeDevice {
///    pub description: String,
/// };
///
/// impl Device for SomeDevice {
///     fn description(&self) -> &str {
///         &*self.description
///     }
/// }
///
/// let device = SomeDevice { description: String::from("some device"), };
///
/// assert_eq!(device.description, device.description());
/// ```

pub trait Device {
    fn description(&self) -> &str;
}
