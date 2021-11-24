use crate::device::Device;

pub struct Thermometer {
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
