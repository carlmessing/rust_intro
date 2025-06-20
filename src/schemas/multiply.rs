use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MultiplyBodyPOST {
    pub a: i32,
    pub b: i32
}