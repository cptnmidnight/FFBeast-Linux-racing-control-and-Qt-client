use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use ffbeast_controller::models::wheel_status::WheelStatus;
use std::collections::{HashSet, HashMap};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};
use directories::ProjectDirs;
use super::models::{KeyMapping, ManagerConfig, Profile, SourceType, TriggerType};

const CONFIG_FILE: &str = "keyboard_profiles.json";

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
                initial_mappings = profile.key_mappings.clone();
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

    pub fn get_config(&self) -> ManagerConfig {
        self.config.lock().unwrap().clone()
    }

    pub fn set_config(&self, config: ManagerConfig) {
        let mut cfg_guard = self.config.lock().unwrap();
        *cfg_guard = config.clone();
        Self::save_config_to_disk(&cfg_guard);

        // Update active mappings from active profile
        if let Some(profile_id) = &cfg_guard.active_profile_id {
            if let Some(profile) = cfg_guard.profiles.iter().find(|p| p.id == *profile_id) {
                let mut m = self.active_mappings.lock().unwrap();
                *m = profile.key_mappings.clone();
            }
        }
    }

    pub fn set_mappings(&self, new_mappings: Vec<KeyMapping>) {
        info!(
            "InputManager: Setting active mappings (PERSISTENT): {}",
            new_mappings.len()
        );
        self.update_active_mappings(new_mappings);
    }

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
            key_mappings: Vec::new(),
            axis_mappings: Vec::new(),
            axis_names: HashMap::new(),
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
                    *mappings = p.key_mappings.clone();
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
            .map(|p| p.key_mappings.clone());

        if let Some(mappings) = profile_mappings {
            config.active_profile_id = Some(id.clone());
            // Update active mappings
            let mut active_m = self.active_mappings.lock().unwrap();
            *active_m = mappings;
            Self::save_config_to_disk(&config);
            Ok(())
        } else {
            Err(format!("Profile with id {} not found", id))
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
        let active_id = config.active_profile_id.clone();

        if let Some(active_id) = active_id {
            if let Some(profile) = config.profiles.iter_mut().find(|p| p.id == active_id) {
                profile.key_mappings = mappings.clone();
            }
            Self::save_config_to_disk(&config);
        }

        // Update live mappings
        let mut m = self.active_mappings.lock().unwrap();
        *m = mappings;
    }

    // --- Processing Loop ---

    pub fn process(&self, status: &WheelStatus) {
        if !*self.active.lock().unwrap() {
            return;
        }

        let inputs_to_process = self.active_mappings.lock().unwrap().clone();
        if inputs_to_process.is_empty() {
            return;
        }

        let mut active_state = self.active_keys.lock().unwrap();
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

            match map.source_type {
                SourceType::Button => {
                    let idx = map.index;
                    if idx < 32 {
                        is_triggered = (status.buttons & (1 << idx)) != 0;
                    }
                }
                SourceType::Axis => {
                    let val = status.adc.get(map.index).cloned().unwrap_or(0);
                    // Scale 12-bit (0-4095) to 16-bit (0-65535)
                    let val_scaled = (val as u32 * 65535) / 4095;
                    
                    // Hysteresis margin (~1% of range) to prevent jitter/flickering
                    let margin = 600;

                    let triggered = if let (Some(min), Some(max)) = (map.threshold_min, map.threshold_max) {
                        // Range based trigger with hysteresis
                        let low = min as u32;
                        let high = max as u32;
                        if was_triggered {
                            val_scaled >= low.saturating_sub(margin) && val_scaled <= high.saturating_add(margin)
                        } else {
                            val_scaled >= low && val_scaled <= high
                        }
                    } else {
                        // Threshold based trigger with hysteresis
                        let thr = map.threshold.unwrap_or(32768) as u32;
                        match map.trigger {
                            TriggerType::High => {
                                if was_triggered { val_scaled >= thr.saturating_sub(margin) }
                                else { val_scaled >= thr }
                            },
                            TriggerType::Low => {
                                if was_triggered { val_scaled <= thr.saturating_add(margin) }
                                else { val_scaled <= thr }
                            },
                            _ => false,
                        }
                    };

                    if triggered != was_triggered {
                        info!(
                            "InputManager [Axis Debug] MapID: {}, Index: {}, Raw: {}, Scaled: {}, Min: {:?}, Max: {:?}, Trigger: {:?}, Result: {}",
                            map.id, map.index, val, val_scaled, map.threshold_min, map.threshold_max, map.trigger, triggered
                        );
                    }
                    is_triggered = triggered;
                }
            };

            let map_key = &map.key;
            if let Some(key_parsed) = parse_key(map_key) {
                 if is_triggered && !was_triggered {
                    info!("InputManager: Pressing Key {} (Source: {:?}[{}]), ID: {}", map_key, map.source_type, map.index, map.id);
                    let _ = enigo.key(key_parsed, Direction::Press);
                    active_state.insert(map.id.clone());
                } else if !is_triggered && was_triggered {
                    info!("InputManager: Releasing Key {} (Source: {:?}[{}]), ID: {}", map_key, map.source_type, map.index, map.id);
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
        if active_state.is_empty() {
            return;
        }

        info!("InputManager: Releasing all keys ({})", active_state.len());
        let mut enigo_guard = self.enigo.lock().unwrap();
        if let Some(enigo) = enigo_guard.as_mut() {
            // Need to retrieve the key names from active_mappings to parse them again?
            // Or just keep the parsed Keys?
            // For now, let's use the mappings list to find keys by ID
            let mappings = self.active_mappings.lock().unwrap();
            for id in active_state.iter() {
                if let Some(m) = mappings.iter().find(|m| &m.id == id) {
                    if let Some(kp) = parse_key(&m.key) {
                        let _ = enigo.key(kp, Direction::Release);
                    }
                }
            }
        }
        active_state.clear();
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
        "A" => Some(Key::Unicode('a')),
        "B" => Some(Key::Unicode('b')),
        "C" => Some(Key::Unicode('c')),
        "D" => Some(Key::Unicode('d')),
        "E" => Some(Key::Unicode('e')),
        "F" => Some(Key::Unicode('f')),
        "G" => Some(Key::Unicode('g')),
        "H" => Some(Key::Unicode('h')),
        "I" => Some(Key::Unicode('i')),
        "J" => Some(Key::Unicode('j')),
        "K" => Some(Key::Unicode('k')),
        "L" => Some(Key::Unicode('l')),
        "M" => Some(Key::Unicode('m')),
        "N" => Some(Key::Unicode('n')),
        "O" => Some(Key::Unicode('o')),
        "P" => Some(Key::Unicode('p')),
        "Q" => Some(Key::Unicode('q')),
        "R" => Some(Key::Unicode('r')),
        "S" => Some(Key::Unicode('s')),
        "T" => Some(Key::Unicode('t')),
        "U" => Some(Key::Unicode('u')),
        "V" => Some(Key::Unicode('v')),
        "W" => Some(Key::Unicode('w')),
        "X" => Some(Key::Unicode('x')),
        "Y" => Some(Key::Unicode('y')),
        "Z" => Some(Key::Unicode('z')),
        "0" => Some(Key::Unicode('0')),
        "1" => Some(Key::Unicode('1')),
        "2" => Some(Key::Unicode('2')),
        "3" => Some(Key::Unicode('3')),
        "4" => Some(Key::Unicode('4')),
        "5" => Some(Key::Unicode('5')),
        "6" => Some(Key::Unicode('6')),
        "7" => Some(Key::Unicode('7')),
        "8" => Some(Key::Unicode('8')),
        "9" => Some(Key::Unicode('9')),
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
        _ => None,
    }
}
