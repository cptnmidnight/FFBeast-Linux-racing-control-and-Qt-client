use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AxisMapping {
    pub name: String,
    pub key_low: String,
    pub key_high: String,
    pub threshold_low: i32,
    pub threshold_high: i32,
}
