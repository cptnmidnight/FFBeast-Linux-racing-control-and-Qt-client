use enigo::{Direction::{Press, Release}, Enigo, Key, Keyboard, Settings};
use std::collections::HashSet;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const CONFIG_FILE: &str = "keyboard_profiles.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMapping {
    pub index: u8,
    pub name: String,
    pub key_low: String,
    pub key_high: String,
    pub btn_low: String,
    pub btn_high: String,
    #[serde(default = "default_min")]
    pub min: u16,
    #[serde(default = "default_max")]
    pub max: u16,
    #[serde(default)]
    pub inverted: bool,
}

fn default_min() -> u16 { 0 }
fn default_max() -> u16 { 65535 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub mappings: Vec<KeyMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct KeyboardConfig {
    active_profile_id: Option<String>,
    profiles: Vec<Profile>,
}

pub struct KeyboardService {
    active: Mutex<bool>,
    config: Mutex<KeyboardConfig>,
    active_keys: Mutex<HashSet<String>>,
    enigo: Mutex<Option<Enigo>>,
}

impl KeyboardService {
    pub fn new() -> Self {
        let enigo_instance = Enigo::new(&Settings::default()).ok();
        
        // Load config
        let mut config = Self::load_config_from_disk();
        if config.profiles.is_empty() {
             // Create default profile
             let default_profile = Profile {
                 id: "default".to_string(),
                 name: "Default".to_string(),
                 mappings: Vec::new(),
             };
             config.profiles.push(default_profile);
             config.active_profile_id = Some("default".to_string());
             Self::save_config_to_disk(&config);
        }

        Self {
            active: Mutex::new(false),
            config: Mutex::new(config),
            active_keys: Mutex::new(HashSet::new()),
            enigo: Mutex::new(enigo_instance),
        }
    }

    fn load_config_from_disk() -> KeyboardConfig {
        if let Ok(content) = fs::read_to_string(CONFIG_FILE) {
            if let Ok(cfg) = serde_json::from_str(&content) {
                return cfg;
            }
        }
        KeyboardConfig::default()
    }

    fn save_config_to_disk(config: &KeyboardConfig) {
        if let Ok(content) = serde_json::to_string_pretty(config) {
            let _ = fs::write(CONFIG_FILE, content);
        }
    }

    pub fn save_current_config(&self) {
        let config = self.config.lock().unwrap();
        Self::save_config_to_disk(&config);
    }

    pub fn set_active(&self, active: bool) {
        let mut guard = self.active.lock().unwrap();
        *guard = active;
        
        if !active {
            let mut keys = self.active_keys.lock().unwrap();
            if let Ok(mut enigo_guard) = self.enigo.lock() {
                if let Some(enigo) = enigo_guard.as_mut() {
                    for key_char in keys.iter() {
                        if let Some(c) = key_char.chars().next() {
                            let _ = enigo.key(Key::Unicode(c), Release);
                        }
                    }
                }
            }
            keys.clear();
        }
    }

    pub fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }

    pub fn process(&self, status: &ffbeast_controller::WheelStatus) {
        if !self.is_active() {
            return;
        }

        let mut enigo_guard = match self.enigo.lock() {
            Ok(g) => g,
            Err(_) => return,
        };
        
        if enigo_guard.is_none() {
             *enigo_guard = Enigo::new(&Settings::default()).ok();
        }

        let enigo = match enigo_guard.as_mut() {
            Some(e) => e,
            None => return, 
        };

        let config = match self.config.lock() {
            Ok(c) => c,
            Err(_) => return,
        };

        let active_profile = match &config.active_profile_id {
            Some(id) => config.profiles.iter().find(|p| p.id == *id),
            None => return,
        };
        
        let mappings = match active_profile {
            Some(p) => &p.mappings,
            None => return,
        };
        
        let mut active_keys_guard = match self.active_keys.lock() {
             Ok(k) => k,
             Err(_) => return,
        };

        for mapping in mappings.iter() {
            let raw_val = match mapping.index {
                0 => status.position as f32, 
                1 => status.adc[0] as f32,
                2 => status.adc[1] as f32,
                3 => status.adc[2] as f32,
                4 => status.adc[3] as f32,
                5 => status.adc[4] as f32,
                _ => 0.0,
            };

            let min = mapping.min as f32;
            let max = mapping.max as f32;
            let range = max - min;
            let mut axis_value = if range > 0.0 {
                (raw_val - min) / range
            } else {
                0.0
            };

            if axis_value < 0.0 { axis_value = 0.0; }
            if axis_value > 1.0 { axis_value = 1.0; }

            if mapping.inverted {
                axis_value = 1.0 - axis_value;
            }

            if !mapping.key_low.is_empty() {
                let char_code = mapping.key_low.chars().next().unwrap();
                let key = Key::Unicode(char_code);
                
                if axis_value < 0.2 {
                    if !active_keys_guard.contains(&mapping.key_low) {
                         let _ = enigo.key(key, Press);
                         active_keys_guard.insert(mapping.key_low.clone());
                    }
                } else if active_keys_guard.contains(&mapping.key_low) {
                     let _ = enigo.key(key, Release);
                     active_keys_guard.remove(&mapping.key_low);
                }
            }

            if !mapping.key_high.is_empty() {
                let char_code = mapping.key_high.chars().next().unwrap();
                let key = Key::Unicode(char_code);

                if axis_value > 0.8 {
                    if !active_keys_guard.contains(&mapping.key_high) {
                         let _ = enigo.key(key, Press);
                         active_keys_guard.insert(mapping.key_high.clone());
                    }
                } else if active_keys_guard.contains(&mapping.key_high) {
                     let _ = enigo.key(key, Release);
                     active_keys_guard.remove(&mapping.key_high);
                }
            }
        }
    }
}

// Commands
#[tauri::command]
pub fn keyboard_service_start(state: tauri::State<std::sync::Arc<KeyboardService>>) -> Result<(), String> {
    state.set_active(true);
    Ok(())
}

#[tauri::command]
pub fn keyboard_service_stop(state: tauri::State<std::sync::Arc<KeyboardService>>) -> Result<(), String> {
    state.set_active(false);
    Ok(())
}

#[tauri::command]
pub fn keyboard_service_restart(state: tauri::State<std::sync::Arc<KeyboardService>>) -> Result<(), String> {
    state.set_active(false);
    std::thread::sleep(std::time::Duration::from_millis(100));
    state.set_active(true);
    Ok(())
}

#[tauri::command]
pub fn keyboard_service_is_active(state: tauri::State<std::sync::Arc<KeyboardService>>) -> Result<bool, String> {
    Ok(state.is_active())
}

#[tauri::command]
pub fn set_keyboard_mapping(state: tauri::State<std::sync::Arc<KeyboardService>>, mappings: Vec<KeyMapping>) -> Result<(), String> {
    // Updates ACTIVE profile
    let mut config = state.config.lock().unwrap();
    let active_id_opt = config.active_profile_id.clone();
    
    if let Some(active_id) = active_id_opt {
        if let Some(profile) = config.profiles.iter_mut().find(|p| p.id == active_id) {
            profile.mappings = mappings;
            // Save inside lock
            KeyboardService::save_config_to_disk(&config);
            return Ok(());
        }
    }
    // If no active profile, create one? Or error?
    // Let's safe guard by creating default if missing
    let default_profile = Profile {
        id: "default".to_string(),
        name: "Default".to_string(),
        mappings: mappings,
    };
    config.profiles.push(default_profile);
    config.active_profile_id = Some("default".to_string());
    KeyboardService::save_config_to_disk(&config);
    Ok(())
}

#[tauri::command]
pub fn get_keyboard_mapping(state: tauri::State<std::sync::Arc<KeyboardService>>) -> Result<Vec<KeyMapping>, String> {
    let config = state.config.lock().unwrap();
    if let Some(active_id) = &config.active_profile_id {
        if let Some(profile) = config.profiles.iter().find(|p| p.id == *active_id) {
            return Ok(profile.mappings.clone());
        }
    }
    Ok(Vec::new())
}

// Check Profile Commands
#[tauri::command]
pub fn list_profiles(state: tauri::State<std::sync::Arc<KeyboardService>>) -> Result<Vec<Profile>, String> {
    let config = state.config.lock().unwrap();
    Ok(config.profiles.clone())
}

#[tauri::command]
pub fn create_profile(state: tauri::State<std::sync::Arc<KeyboardService>>, name: String) -> Result<String, String> {
    let mut config = state.config.lock().unwrap();
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
    let profile = Profile {
        id: id.clone(),
        name,
        mappings: Vec::new(),
    };
    config.profiles.push(profile);
    // Set as active instantly? No, user chooses.
    KeyboardService::save_config_to_disk(&config);
    Ok(id)
}

#[tauri::command]
pub fn delete_profile(state: tauri::State<std::sync::Arc<KeyboardService>>, id: String) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    config.profiles.retain(|p| p.id != id);
    if config.active_profile_id.as_ref() == Some(&id) {
        config.active_profile_id = config.profiles.first().map(|p| p.id.clone());
    }
    KeyboardService::save_config_to_disk(&config);
    Ok(())
}

#[tauri::command]
pub fn set_active_profile(state: tauri::State<std::sync::Arc<KeyboardService>>, id: String) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    if config.profiles.iter().any(|p| p.id == id) {
        config.active_profile_id = Some(id);
        KeyboardService::save_config_to_disk(&config);
        Ok(())
    } else {
        Err("Profile not found".to_string())
    }
}

#[tauri::command]
pub fn get_active_profile_id(state: tauri::State<std::sync::Arc<KeyboardService>>) -> Result<Option<String>, String> {
    let config = state.config.lock().unwrap();
    Ok(config.active_profile_id.clone())
}
