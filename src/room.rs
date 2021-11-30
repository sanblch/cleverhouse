use crate::device::DeviceType;

pub struct Room {
    pub name: String,
    _devices: Vec<DeviceType>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            name,
            _devices: Vec::new(),
        }
    }

    pub fn add_device(&self, _device_type: DeviceType) -> Result<(), Error> {
        todo!()
    }

    pub fn get_device(&self, _device_name: String) -> Option<DeviceType> {
        todo!()
    }

    pub fn remove_device(&self, _device_name: String) -> Result<(), Error> {
        todo!()
    }

    pub fn list_devices(&self) -> Vec<String> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    AlreadyExist,
    NameUsed,
    NotExist,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::AlreadyExist => write!(f, "Room already contains a device with the name"),
            Error::NameUsed => write!(f, "Room already contains another device with the name"),
            Error::NotExist => write!(f, "Device doesn't exist"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::smartsocket::SmartSocket;
    use crate::devices::thermometer::Thermometer;

    fn test_empty_room() -> Room {
        Room::new(String::from("MyRoom"))
    }

    fn test_room() -> Room {
        let room = test_empty_room();
        let _res1 = room.add_device(DeviceType::SmartSocket(test_smartsocket()));
        let _res2 = room.add_device(DeviceType::Thermometer(test_thermometer()));
        room
    }

    fn test_smartsocket() -> SmartSocket {
        SmartSocket {
            name: String::from("SmartSocket"),
            description: String::from("A type of socket"),
        }
    }

    fn test_thermometer() -> Thermometer {
        Thermometer {
            name: String::from("Thermometer"),
            description: String::from("A type of thermometer"),
        }
    }

    #[test]
    fn create_room() {
        let room = test_empty_room();
        assert_eq!(room.name, "MyRoom");
        assert_eq!(room.list_devices(), Vec::<String>::new());
    }

    #[test]
    fn add_device() {
        let room = test_empty_room();
        assert_eq!(
            room.add_device(DeviceType::SmartSocket(test_smartsocket())),
            Ok(())
        );
        assert_eq!(
            room.add_device(DeviceType::Thermometer(test_thermometer())),
            Ok(())
        );
        assert_eq!(
            room.add_device(DeviceType::SmartSocket(test_smartsocket())),
            Err(Error::AlreadyExist)
        );
        assert_eq!(
            room.add_device(DeviceType::Thermometer(test_thermometer())),
            Err(Error::AlreadyExist)
        );
        assert_eq!(
            room.add_device(DeviceType::SmartSocket(SmartSocket {
                name: String::from("Thermometer"),
                description: String::from("A type of socket"),
            })),
            Err(Error::NameUsed)
        );
        assert_eq!(
            room.add_device(DeviceType::Thermometer(Thermometer {
                name: String::from("SmartSocket"),
                description: String::from("A type of thermometer"),
            })),
            Err(Error::NameUsed)
        );
        assert_eq!(
            room.list_devices(),
            vec![test_smartsocket().name, test_thermometer().name]
        );
    }

    #[test]
    fn remove_device() {
        let room = test_room();
        assert_eq!(
            room.remove_device(String::from(test_smartsocket().name)),
            Ok(())
        );
        assert_eq!(
            room.remove_device(String::from(test_thermometer().name)),
            Ok(())
        );
        assert_eq!(
            room.remove_device(String::from(test_thermometer().name)),
            Err(Error::NotExist)
        );
        assert_eq!(room.list_devices(), Vec::<String>::new());
    }

    #[test]
    fn get_device() {
        let room = test_room();
        match room.get_device(test_smartsocket().name) {
            Some(u) => match u {
                DeviceType::SmartSocket(device) => {
                    assert_eq!(test_smartsocket().name, device.name);
                }
                DeviceType::Thermometer(_device) => unreachable!(),
            },
            None => unreachable!(),
        };
        match room.get_device(test_thermometer().name) {
            Some(u) => match u {
                DeviceType::SmartSocket(_device) => unreachable!(),
                DeviceType::Thermometer(device) => {
                    assert_eq!(test_thermometer().name, device.name);
                }
            },
            None => unreachable!(),
        };
        assert_eq!(room.get_device(String::from("MyDevice")).is_none(), true);
    }
}
