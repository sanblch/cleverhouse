use crate::device::DeviceType;

pub struct Room {
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
