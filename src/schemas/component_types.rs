use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Operands {
    pub a: f64,
    pub b: f64
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub result: f64
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub error: String
}

#[derive(Serialize, Deserialize)]
pub struct InfoZ {
    pub title: String,
    pub version: String,
    pub description: String
}