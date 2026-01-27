use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use ffbeast_controller::models::wheel_status::WheelStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};
use directories::ProjectDirs;

const CONFIG_FILE: &str = "keyboard_profiles.json";

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyMapping {
    pub id: String,
    pub source_type: String, // "button", "axis"
    pub index: usize,
    pub trigger: String, // "press", "high", "low"
    pub key: String,     // "A", "SPACE", etc.
    pub threshold: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub mappings: Vec<KeyMapping>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ManagerConfig {
    pub active_profile_id: Option<String>,
    pub profiles: Vec<Profile>,
}

pub struct InputManager {
    active: Mutex<bool>,
    // The currently active mappings used by the process loop
    active_mappings: Mutex<Vec<KeyMapping>>,
    // The persistent configuration (profiles)
    config: Mutex<ManagerConfig>,
    // State of currently pressed keys
    active_keys: Mutex<HashSet<String>>,
    // Input Simulator
    enigo: Mutex<Option<Enigo>>,
}

impl InputManager {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default()).ok();
        if enigo.is_none() {
            warn!("InputManager: Failed to initialize Enigo. Input simulation disabled.");
        }

        let config = Self::load_config_from_disk();
        let mut initial_mappings = Vec::new();

        // Load active mappings from profile if one is active
        if let Some(profile_id) = &config.active_profile_id {
            if let Some(profile) = config.profiles.iter().find(|p| p.id == *profile_id) {
                initial_mappings = profile.mappings.clone();
            }
        }

        Self {
            active: Mutex::new(false),
            active_mappings: Mutex::new(initial_mappings),
            config: Mutex::new(config),
            active_keys: Mutex::new(HashSet::new()),
            enigo: Mutex::new(enigo),
        }
    }

    pub fn from_saved_config() -> Self {
        Self::new()
    }

    fn get_config_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("com", "sodevs", "InputManager") {
            let base_dir = proj_dirs.config_dir(); // ~/.config/SODevsGameLauncher or AppData/Roaming/...
            let path = base_dir.to_path_buf();           

            if !path.exists() {
                let _ = fs::create_dir_all(&path);
            }
            path.join(CONFIG_FILE)
        } else {
            PathBuf::from(CONFIG_FILE)
        }
    }

    fn load_config_from_disk() -> ManagerConfig {
        let path = Self::get_config_path();
        if let Ok(content) = fs::read_to_string(&path) {
            match serde_json::from_str::<ManagerConfig>(&content) {
                Ok(cfg) => return cfg,
                Err(e) => warn!("Failed to parse {:?}: {}", path, e),
            }
        }
        ManagerConfig::default()
    }

    fn save_config_to_disk(config: &ManagerConfig) {
        let path = Self::get_config_path();
        if let Ok(content) = serde_json::to_string_pretty(config) {
            let _ = fs::write(&path, content);
        }
    }

    // --- Active State Management ---

    pub fn set_mappings(&self, new_mappings: Vec<KeyMapping>) {
        info!(
            "InputManager: Setting active mappings (PERSISTENT): {}",
            new_mappings.len()
        );
        self.update_active_mappings(new_mappings);
    }

    pub fn set_active(&self, active: bool) {
        info!("InputManager: Setting active = {}", active);
        *self.active.lock().unwrap() = active;
        if !active {
            self.release_all_keys();
        } else {
            let count = self.active_mappings.lock().unwrap().len();
            info!("InputManager: Activated with {} mappings", count);
        }
    }

    pub fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }

    fn release_all_keys(&self) {
        let mut active_state = self.active_keys.lock().unwrap();
        let mappings = self.active_mappings.lock().unwrap();
        let mut enigo_guard = self.enigo.lock().unwrap();

        if let Some(enigo) = enigo_guard.as_mut() {
            for id in active_state.iter() {
                if let Some(map) = mappings.iter().find(|m| &m.id == id) {
                    if let Some(vk) = parse_key(&map.key) {
                        let _ = enigo.key(vk, Direction::Release);
                    }
                }
            }
        }
        active_state.clear();
    }

    // --- Profile Management ---

    pub fn list_profiles(&self) -> Vec<Profile> {
        self.config.lock().unwrap().profiles.clone()
    }

    pub fn create_profile(&self, name: String) -> String {
        let mut config = self.config.lock().unwrap();
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let profile = Profile {
            id: id.clone(),
            name,
            mappings: Vec::new(),
        };
        config.profiles.push(profile);
        Self::save_config_to_disk(&config);
        id
    }

    pub fn delete_profile(&self, id: String) {
        let mut config = self.config.lock().unwrap();
        config.profiles.retain(|p| p.id != id);

        let active_id = config.active_profile_id.clone();
        if active_id.as_ref() == Some(&id) {
            let new_active = config.profiles.first().map(|p| p.id.clone());
            config.active_profile_id = new_active.clone();

            // Also update active mappings
            let mut mappings = self.active_mappings.lock().unwrap();
            if let Some(new_id) = new_active {
                if let Some(p) = config.profiles.iter().find(|p| p.id == new_id) {
                    *mappings = p.mappings.clone();
                } else {
                    mappings.clear();
                }
            } else {
                mappings.clear();
            }
        }
        Self::save_config_to_disk(&config);
    }

    pub fn set_active_profile(&self, id: String) -> Result<(), String> {
        let mut config = self.config.lock().unwrap();

        // Check if profile exists
        let profile_mappings = config
            .profiles
            .iter()
            .find(|p| p.id == id)
            .map(|p| p.mappings.clone());

        if let Some(mappings) = profile_mappings {
            config.active_profile_id = Some(id.clone());
            // Update active mappings
            let mut active_m = self.active_mappings.lock().unwrap();
            *active_m = mappings;
            Self::save_config_to_disk(&config);
            Ok(())
        } else {
            Err("Profile not found".to_string())
        }
    }

    pub fn get_active_profile_id(&self) -> Option<String> {
        self.config.lock().unwrap().active_profile_id.clone()
    }

    pub fn get_active_mappings(&self) -> Vec<KeyMapping> {
        self.active_mappings.lock().unwrap().clone()
    }

    // Allows updating the mappings of the current profile (or creates one)
    pub fn update_active_mappings(&self, mappings: Vec<KeyMapping>) {
        self.release_all_keys();
        let mut config = self.config.lock().unwrap();
        // If we have an active profile, update it
        // If we have an active profile, update it
        let active_id = config.active_profile_id.clone();

        if let Some(active_id) = active_id {
            if let Some(profile) = config.profiles.iter_mut().find(|p| p.id == active_id) {
                profile.mappings = mappings.clone();
            }
        } else {
            // Create default if none exists
            let id = "default".to_string();
            let profile = Profile {
                id: id.clone(),
                name: "Default".to_string(),
                mappings: mappings.clone(),
            };
            config.profiles.push(profile);
            config.active_profile_id = Some(id);
        }
        Self::save_config_to_disk(&config);

        // Update live mappings
        let mut m = self.active_mappings.lock().unwrap();
        *m = mappings;
    }

    // --- Processing Loop ---

    pub fn process(&self, status: &WheelStatus) {
        if !self.is_active() {
            return;
        }

        let inputs_to_process = {
            let m = self.active_mappings.lock().unwrap();
            m.clone()
        };

        // ... same processing logic ...
        // ... same processing logic ...
        let mut active_state = self.active_keys.lock().unwrap();
        // let mut enigo_guard = self.enigo.lock().unwrap();
        // Avoid keeping the lock for too long, but we need it to press keys
        // We will lock it only when action is needed if possible? No, we need it to be mutable.
        let mut enigo_guard = self.enigo.lock().unwrap();
        let enigo = match enigo_guard.as_mut() {
            Some(e) => e,
            None => {
                warn!("InputManager: Enigo instance not available");
                return;
            },
        };

        for map in inputs_to_process {
            let was_triggered = active_state.contains(&map.id); // Check previous state
            let mut is_triggered = false;

            match map.source_type.as_str() {
                "button" => {
                    let idx = map.index;
                    if idx < 32 {
                        is_triggered = (status.buttons & (1 << idx)) != 0;
                    }
                }
                "axis" => {
                    let val = status.adc.get(map.index).cloned().unwrap_or(0);
                    // Scale 12-bit (0-4095) to match common usage, previously scaling to 15-bit was confusing if threshold is %
                    // Assuming threshold is also in 0-4095 scale or 0-100%?
                    // The frontend sends raw values usually. Let's assume threshold is RAW 0-4095 for now based on typical usage.
                    // If the original code scaled to 32767, let's keep it but LOG it.
                    let val_scaled = (val as i32 * 32767) / 4095;
                    let thr = map.threshold.unwrap_or(16383); // ~50% of 32767

                    let triggered = match map.trigger.as_str() {
                        "high" => val_scaled > thr,
                        "low" => val_scaled < thr,
                         _ => false,
                    };

                    // DEBUG LOGS FOR AXIS
                    // Only log if state CHANGES or periodically? continuous logging kills perf.
                    // Let's log only on edge detection later, or if it's suspected to differ from expected.
                    // If the user wants to debug "why it is not triggering", we should log when it is CLOSE or any change?
                    // For now, let's log transitions.

                    if triggered != was_triggered {
                        info!(
                            "InputManager [Axis Debug] MapID: {}, Index: {}, Raw: {}, Scaled: {}, Threshold: {}, Trigger: {}, Result: {}",
                            map.id, map.index, val, val_scaled, thr, map.trigger, triggered
                        );
                    }
                    is_triggered = triggered;
                }
                _ => {}
            };

            let map_key = &map.key;
            if let Some(key_parsed) = parse_key(map_key) {
                 if is_triggered && !was_triggered {
                    info!("InputManager: Pressing Key {} (Source: {}[{}]), ID: {}", map_key, map.source_type, map.index, map.id);
                    let _ = enigo.key(key_parsed, Direction::Press);
                    active_state.insert(map.id.clone());
                } else if !is_triggered && was_triggered {
                    info!("InputManager: Releasing Key {} (Source: {}[{}]), ID: {}", map_key, map.source_type, map.index, map.id);
                    let _ = enigo.key(key_parsed, Direction::Release);
                    active_state.remove(&map.id);
                }
            } else {
                 if is_triggered && !was_triggered {
                     warn!("InputManager: Invalid key '{}' configured for ID {}", map_key, map.id);
                 }
            }
        }
    }
}

pub fn parse_key(k: &str) -> Option<Key> {
    let upper = k.to_uppercase();
    match upper.as_str() {
        "SPACE" => Some(Key::Space),
        "ENTER" | "RETURN" => Some(Key::Return),
        "TAB" => Some(Key::Tab),
        "ESC" | "ESCAPE" => Some(Key::Escape),
        "BACKSPACE" => Some(Key::Backspace),
        "UP" => Some(Key::UpArrow),
        "DOWN" => Some(Key::DownArrow),
        "LEFT" => Some(Key::LeftArrow),
        "RIGHT" => Some(Key::RightArrow),
        "SHIFT" => Some(Key::Shift),
        "CTRL" | "CONTROL" => Some(Key::Control),
        "ALT" => Some(Key::Alt),
        "CAPSLOCK" => Some(Key::CapsLock),
        "META" | "SUPER" | "WIN" => Some(Key::Meta),
        "OPTION" => Some(Key::Option),
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        c if c.len() == 1 => {
            let ch = c.chars().next().unwrap();
            Some(Key::Unicode(ch.to_ascii_lowercase()))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key() {
        assert_eq!(parse_key("SPACE"), Some(Key::Space));
        assert_eq!(parse_key("a"), Some(Key::Unicode('a')));
        assert_eq!(parse_key("A"), Some(Key::Unicode('a'))); // checks lowercasing
        assert_eq!(parse_key("F1"), Some(Key::F1));
        assert!(parse_key("INVALID").is_none());
    }
}
