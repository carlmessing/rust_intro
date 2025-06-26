use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Operands {
    pub a: i32,
    pub b: i32
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub result: i32
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