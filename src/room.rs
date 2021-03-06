use crate::device::DeviceType;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Room {
    devices: HashMap<String, DeviceType>,
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

impl Room {
    pub fn new() -> Self {
        Room {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, name: &str, r#type: DeviceType) -> Result<(), Error> {
        match self.devices.entry(name.to_owned()) {
            Entry::Occupied(item) => {
                if std::mem::discriminant(item.get()) == std::mem::discriminant(&r#type) {
                    Err(Error::AlreadyExist)
                } else {
                    Err(Error::NameUsed)
                }
            }
            Entry::Vacant(item) => {
                item.insert(r#type);
                Ok(())
            }
        }
    }

    pub fn get_device(&self, name: &str) -> Option<&DeviceType> {
        self.devices.get(&name.to_owned())
    }

    pub fn remove_device(&mut self, name: &str) -> Result<DeviceType, Error> {
        self.devices
            .remove(&name.to_string())
            .ok_or(Error::NotExist)
    }

    pub fn list_devices(&self) -> impl Iterator<Item = &str> {
        self.devices.iter().map(|c| c.0.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    AlreadyExist,
    ExactMatch,
    NameUsed,
    NotExist,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::AlreadyExist => write!(f, "Room already contains a device with the name"),
            Error::ExactMatch => write!(f, "Room already contains exact to_owned of the device"),
            Error::NameUsed => write!(f, "Room already contains another device with the name"),
            Error::NotExist => write!(f, "Device doesn't exist"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::smartsocket::simple::SmartSocket;
    use crate::devices::thermometer::simple::Thermometer;

    const S: &str = "SmartSocket";
    const T: &str = "Thermometer";

    fn test_empty_room() -> Room {
        Room::new()
    }

    fn test_room() -> Room {
        let mut room = test_empty_room();
        let _res1 = room.add_device(S, DeviceType::SmartSocket(test_smartsocket()));
        let _res2 = room.add_device(T, DeviceType::Thermometer(test_thermometer()));
        room
    }

    fn test_smartsocket() -> SmartSocket {
        SmartSocket::new("A type of socket".to_string())
    }

    fn test_thermometer() -> Thermometer {
        Thermometer::new("A type of thermometer".to_string())
    }

    #[test]
    fn create_room() {
        let room = test_empty_room();
        assert_eq!(
            room.list_devices().collect::<Vec<&str>>(),
            Vec::<String>::new()
        );
    }

    #[test]
    fn add_device() {
        let mut room = test_empty_room();
        assert_eq!(
            room.add_device(S, DeviceType::SmartSocket(test_smartsocket())),
            Ok(())
        );
        assert_eq!(
            room.add_device(T, DeviceType::Thermometer(test_thermometer())),
            Ok(())
        );
        assert_eq!(
            room.add_device(S, DeviceType::SmartSocket(test_smartsocket())),
            Err(Error::AlreadyExist)
        );
        assert_eq!(
            room.add_device(T, DeviceType::Thermometer(test_thermometer())),
            Err(Error::AlreadyExist)
        );
        assert_eq!(
            room.add_device(
                T,
                DeviceType::SmartSocket(SmartSocket::new("A type of socket".to_string()))
            ),
            Err(Error::NameUsed)
        );
        assert_eq!(
            room.add_device(
                S,
                DeviceType::Thermometer(Thermometer::new("A type of thermometer".to_string()))
            ),
            Err(Error::NameUsed)
        );
        let mut list = room.list_devices().collect::<Vec<&str>>();
        list.sort();
        assert_eq!(list, vec![S, T]);
    }

    #[test]
    fn remove_device() {
        let mut room = test_room();
        assert_eq!(
            room.remove_device(S),
            Ok(DeviceType::SmartSocket(test_smartsocket()))
        );
        assert_eq!(
            room.remove_device(T),
            Ok(DeviceType::Thermometer(test_thermometer()))
        );
        assert_eq!(room.remove_device(T), Err(Error::NotExist));
        assert_eq!(
            room.list_devices().collect::<Vec<&str>>(),
            Vec::<String>::new()
        );
    }

    #[test]
    fn get_device() {
        let room = test_room();
        match room.get_device(S) {
            Some(u) => match u {
                DeviceType::SmartSocket(device) => {
                    assert_eq!(device.description, "A type of socket".to_string());
                }
                DeviceType::Thermometer(_device) => unreachable!(),
            },
            None => unreachable!(),
        };
        match room.get_device(T) {
            Some(u) => match u {
                DeviceType::SmartSocket(_device) => unreachable!(),
                DeviceType::Thermometer(device) => {
                    assert_eq!(device.description, "A type of thermometer".to_string());
                }
            },
            None => unreachable!(),
        };
        assert_eq!(room.get_device("MyDevice").is_none(), true);
    }
}
