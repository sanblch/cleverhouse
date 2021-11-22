struct House {
    _name: String,
    _rooms: Vec<Room>,
}

impl House {
    fn new(_name: String) -> Self {
        House {
            _name,
            _rooms: Vec::new(),
        }
    }

    fn _create_room(&self, _name: String) -> Result<&'static mut Room, String> {
        todo!()
    }

    fn _remove_room(&self, _name: String) -> Result<(), String> {
        todo!()
    }

    fn _list_rooms(&self) -> Vec<String> {
        todo!()
    }

    fn _report(&self) {
        todo!()
    }
}

struct Room {
    _name: String,
    _devices: Vec<DeviceType>,
}

impl Room {
    fn _add_device(&self, _device_type: DeviceType) -> Result<(), String> {
        todo!()
    }

    fn _get_device(&self, _device_name: String) -> Result<DeviceType, String> {
        todo!()
    }

    fn _remove_device(&self, _device_name: String) -> Result<(), String> {
        todo!()
    }

    fn _list_devices(&self) -> Result<Vec<String>, String> {
        todo!()
    }
}

pub enum DeviceType {
    SmartSocket(String, String),
    Thermometer(String, String),
}

pub trait Device {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

struct Thermometer {
    name: String,
    description: String,
}

impl Device for Thermometer {
    fn name(&self) -> &str {
        &*self.name
    }
    fn description(&self) -> &str {
        &*self.description
    }
}

impl Thermometer {
    fn _temperature() -> f64 {
        todo!()
    }
}

struct SmartSocket {
    name: String,
    description: String,
}

impl Device for SmartSocket {
    fn name(&self) -> &str {
        &*self.name
    }
    fn description(&self) -> &str {
        &*self.description
    }
}

impl SmartSocket {
    fn _on() {
        todo!()
    }
    fn _off() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_house() {
        let mut _house = House::new(String::from("MyHouse"));
    }
}
