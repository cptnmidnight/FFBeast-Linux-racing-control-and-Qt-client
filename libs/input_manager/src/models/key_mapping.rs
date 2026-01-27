use serde::{Deserialize, Serialize};
use super::source_type::SourceType;
use super::trigger_type::TriggerType;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyMapping {
    pub id: String,
    pub source_type: SourceType,
    pub index: usize,
    pub trigger: TriggerType,
    pub key: String,
    pub button: Option<String>,
    pub threshold: Option<i32>, // Deprecated
    pub threshold_min: Option<i32>,
    pub threshold_max: Option<i32>,
}
