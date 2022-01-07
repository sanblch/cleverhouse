use crate::device::Device;
use super::{server::SmartSocketStream, api::{Status, Request, SmartSocketResult, SmartSocketError}};

pub struct SmartSocket {
    pub description: String,
    pub client: SmartSocketStream,
}

impl Device for SmartSocket {
    fn description(&self) -> &str {
        &*self.description
    }
}

impl SmartSocket {
    pub fn new(client: SmartSocketStream, description: String) -> Self {
        SmartSocket {
            description,
            client,
        }
    }

    pub fn is_on(&self) -> std::io::Result<Status> {
        self.client.send(Request::Status)?;
        let res = self.client.receive::<SmartSocketResult>()?;
        match res {
            Ok(response) => {
                match response {
                    Status(status) => {
                        Ok(status)
                    },
                    _ => {
                        Err(SmartSocketError::Unexpected)
                    }
                }
            },
            Err(e) => {
                Err(std::io::Error::new(std::io::ErrorKind::Other, e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smartsocket() {
        let client = SmartSocketStream::connect("127.0.0.1:12345").unwrap();
        let mut smartsocket = SmartSocket::new(client, String::from("a smart socket"));
    }
}
