use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Healthcheck {
    pub message: String,
    pub version: String
}