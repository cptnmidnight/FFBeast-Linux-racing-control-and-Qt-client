use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AxisMapping {
    pub name: String,
    pub key: String,
    pub button: String,
    pub threshold_low: i32,
    pub threshold_high: i32,
}
