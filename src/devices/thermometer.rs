use crate::device::Device;

pub struct Thermometer {
    pub name: String,
    pub description: String,
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
    pub fn temperature(&self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermometer() {
        let thermometer = Thermometer {
            name: String::from("Thermometer"),
            description: String::from("a thermometer"),
        };
        thermometer.temperature();
    }
}
