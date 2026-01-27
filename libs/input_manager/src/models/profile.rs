use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::key_mapping::KeyMapping;
use super::axis_mapping::AxisMapping;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub key_mappings: Vec<KeyMapping>,
    pub axis_mappings: Vec<AxisMapping>,
    pub axis_names: HashMap<usize, String>,
}
