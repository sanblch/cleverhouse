struct House {
    _name: String,
    _rooms: Vec<Room>,
}

impl House {
    fn new(_name: String) -> Self {
        House { _name, _rooms: Vec::new() }
    }

    fn _create_room(&self, _name: String) -> Result<&'static mut Room, String> {
        todo!()
    }

    fn _remove_room(&self, _name: String) -> Result<(), String> {
        todo!()
    }

    fn _rooms(&self) -> Vec<String> {
        todo!()
    }

    fn _add_device(&self, _room_name: String, _device_name: String, _device_type: DeviceType) -> Result<&mut dyn Device, String> {
        todo!()
    }

    fn _get_device(&self, _room_name: String, _device_name: String) -> Result<&'static mut dyn Device, String> {
        todo!()
    }

    fn _remove_device(&self, _room_name: String, _device_name: String) -> Result<(), String> {
        todo!()
    }

    fn _devices(&self, _room_name: String) -> Result<Vec<String>, String> {
        todo!()
    }

    fn _report(&self) {
        todo!()
    }
}

struct Room {
    _name: String,
    _devices: Vec<Box<dyn Device>>,
}

pub enum DeviceType {
    SmartSocket,
    Thermometer,
}

pub trait Device {
    fn name(&self) -> &str;
    fn device_type(&self) -> DeviceType;
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
    fn device_type(&self) -> DeviceType {
        DeviceType::Thermometer
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
    fn device_type(&self) -> DeviceType {
        DeviceType::SmartSocket
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

fn main() {
    let mut _house = House::new(String::from("MyHouse"));
}
