use crate::device::Device;

pub struct SmartSocket {
    pub name: String,
    pub description: String,
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
    pub fn on(&self) {
        todo!()
    }
    pub fn off(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smartsocket() {
        let smartsocket = SmartSocket {
            name: String::from("SmartSocket"),
            description: String::from("a smart socket"),
        };
        smartsocket.on();
        smartsocket.off();
    }
}
