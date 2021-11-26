use crate::devices::smartsocket::SmartSocket;
use crate::devices::thermometer::Thermometer;

pub enum DeviceType {
    SmartSocket(SmartSocket),
    Thermometer(Thermometer),
}

/// Here is a use example
/// ```
/// use cleverhouse_by_akrutikov::device::Device;
/// pub struct SomeDevice {
///    pub name: String,
///    pub description: String,
/// };
///
/// impl Device for SomeDevice {
///     fn name(&self) -> &str {
///         &*self.name
///     }
///     fn description(&self) -> &str {
///         &*self.description
///     }
/// }
///
/// let device = SomeDevice { name: String::from("SomeDevice"), description: String::from("some device"),};
///
/// assert_eq!(device.name, device.name());
/// assert_eq!(device.description, device.description());
/// ```

pub trait Device {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}
