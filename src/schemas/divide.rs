use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DivideBodyPOST {
    pub a: i32,
    pub b: i32
}