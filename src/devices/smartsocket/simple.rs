use crate::device::Device;

#[derive(Debug, PartialEq)]
pub struct SmartSocket {
    pub description: String,
    on: bool,
}

impl Device for SmartSocket {
    fn description(&self) -> &str {
        &*self.description
    }
}

impl SmartSocket {
    pub fn new(description: String) -> Self {
        SmartSocket {
            description,
            on: false,
        }
    }

    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn on(&mut self) {
        self.on = true;
    }
    pub fn off(&mut self) {
        self.on = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smartsocket() {
        let mut smartsocket = SmartSocket::new(String::from("a smart socket"));
        assert_eq!(smartsocket.is_on(), false);
        smartsocket.on();
        assert_eq!(smartsocket.is_on(), true);
        smartsocket.off();
        assert_eq!(smartsocket.is_on(), false);
    }
}
