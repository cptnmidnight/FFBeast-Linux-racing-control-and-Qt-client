mod models;
mod input_manager;

pub use input_manager::{InputManager, parse_key};
pub use models::{AxisMapping, KeyMapping, ManagerConfig, Profile, SourceType, TriggerType};
