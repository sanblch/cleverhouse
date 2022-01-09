use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmartSocketError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    On,
    Off,
    Status,
    Power,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    On(Status),
    Off(Status),
    Status(Status),
    Power(f64),
}

pub type SmartSocketResult = Result<Response, String>;
