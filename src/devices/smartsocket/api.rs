use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    On,
    Off,
    Status,
    Power,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Status {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Error(String),
    Status(Status),
    Power(f64),
}
