use crate::device::Device;

pub struct SmartSocket {
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
