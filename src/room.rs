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

    pub fn add_device(&self, _device_type: DeviceType) -> Result<(), String> {
        todo!()
    }

    pub fn get_device(&self, _device_name: String) -> Result<DeviceType, String> {
        todo!()
    }

    pub fn remove_device(&self, _device_name: String) -> Result<(), String> {
        todo!()
    }

    pub fn list_devices(&self) -> Vec<String> {
        todo!()
    }
}

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
    fn add_remove_device() {
        let room = test_room();
        assert_eq!(
            room.list_devices(),
            vec![test_smartsocket().name, test_thermometer().name]
        );
        let _res1 = room.remove_device(String::from(test_smartsocket().name));
        let _res2 = room.remove_device(String::from(test_thermometer().name));
        assert_eq!(room.list_devices(), Vec::<String>::new());
    }

    #[test]
    fn get_device() {
        let room = test_room();
        match room.get_device(test_smartsocket().name) {
            Ok(u) => match u {
                DeviceType::SmartSocket(device) => {
                    assert_eq!(test_smartsocket().name, device.name);
                }
                DeviceType::Thermometer(_device) => unreachable!(),
            },
            Err(_e) => unreachable!(),
        };
        match room.get_device(test_thermometer().name) {
            Ok(u) => match u {
                DeviceType::SmartSocket(_device) => unreachable!(),
                DeviceType::Thermometer(device) => {
                    assert_eq!(test_thermometer().name, device.name);
                }
            },
            Err(_e) => unreachable!(),
        };
    }
}
