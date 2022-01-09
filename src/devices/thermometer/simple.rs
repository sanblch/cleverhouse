use crate::device::Device;
use std::f64::NAN;

#[derive(Debug)]
pub struct Thermometer {
    pub description: String,
    temperature: f64,
}

impl Device for Thermometer {
    fn description(&self) -> &str {
        &*self.description
    }
}

impl PartialEq for Thermometer {
    fn eq(&self, other: &Thermometer) -> bool {
        self.description == other.description
            && (self.temperature == other.temperature
                || self.temperature.is_nan() && other.temperature.is_nan())
    }
}

impl Thermometer {
    pub fn new(description: String) -> Self {
        Thermometer {
            description,
            temperature: NAN,
        }
    }

    pub fn set(&mut self, temperature: f64) {
        self.temperature = temperature;
    }

    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermometer() {
        let mut thermometer = Thermometer::new("a thermometer".to_string());
        assert_eq!(thermometer.temperature().is_nan(), true);
        thermometer.set(100.0);
        assert_eq!(thermometer.temperature(), 100.0);
    }
}
