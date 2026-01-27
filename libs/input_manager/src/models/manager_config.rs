use serde::{Deserialize, Serialize};
use super::profile::Profile;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ManagerConfig {
    pub active_profile_id: Option<String>,
    pub profiles: Vec<Profile>,
}
