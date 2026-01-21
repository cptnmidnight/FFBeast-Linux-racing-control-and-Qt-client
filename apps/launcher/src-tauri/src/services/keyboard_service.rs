use enigo::Keyboard;
use std::collections::HashSet;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMapping {
    pub index: u8,
    pub name: String,
    pub key_low: String,
    pub key_high: String,
    pub btn_low: String,
    pub btn_high: String,
    // Add other fields from hardware config if needed
}

pub struct KeyboardService {
    active: Mutex<bool>,
    mappings: Mutex<Vec<KeyMapping>>,
    active_keys: Mutex<HashSet<String>>,
}

impl KeyboardService {
    pub fn new() -> Self {
        Self {
            active: Mutex::new(false),
            mappings: Mutex::new(Vec::new()),
            active_keys: Mutex::new(HashSet::new()),
        }
    }

    pub fn set_mappings(&self, mappings: Vec<KeyMapping>) {
        let mut guard = self.mappings.lock().unwrap();
        *guard = mappings;
    }

    pub fn set_active(&self, active: bool) {
        let mut guard = self.active.lock().unwrap();
        *guard = active;
        
        // If stopping, ensure all keys are released
        if !active {
            // Implementation to release all keys would go here
            // using Enigo or similar library
            let mut keys = self.active_keys.lock().unwrap();
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

        let mappings = self.mappings.lock().unwrap();
        let mut active_keys_guard = self.active_keys.lock().unwrap();
        let mut enigo = match enigo::Enigo::new(&enigo::Settings::default()) {
            Ok(e) => e,
            Err(e) => {
                // Log warning once to avoid spamming
                // tracing::warn!("Failed to init Enigo: {}", e); 
                return;
            }
        };

        for mapping in mappings.iter() {
            let axis_value = match mapping.index {
                0 => status.position as f32 / 900.0, // Normalize roughly -1 to 1 based on 900deg default
                1 => status.adc[0] as f32 / 1023.0,
                2 => status.adc[1] as f32 / 1023.0,
                3 => status.adc[2] as f32 / 1023.0,
                4 => status.adc[3] as f32 / 1023.0,
                5 => status.adc[4] as f32 / 1023.0,
                _ => 0.0,
            };

            // Trigger Low Key (< 20%)
            if !mapping.key_low.is_empty() {
                let key = enigo::Key::Unicode(mapping.key_low.chars().next().unwrap()); // Changed to Unicode
                if axis_value < 0.2 {
                    if !active_keys_guard.contains(&mapping.key_low) {
                         let _ = enigo.key(key, enigo::Direction::Press);
                         active_keys_guard.insert(mapping.key_low.clone());
                    }
                } else if active_keys_guard.contains(&mapping.key_low) {
                     let _ = enigo.key(key, enigo::Direction::Release);
                     active_keys_guard.remove(&mapping.key_low);
                }
            }

             // Trigger High Key (> 80%)
            if !mapping.key_high.is_empty() {
                let key = enigo::Key::Unicode(mapping.key_high.chars().next().unwrap()); // Changed to Unicode
                if axis_value > 0.8 {
                    if !active_keys_guard.contains(&mapping.key_high) {
                         let _ = enigo.key(key, enigo::Direction::Press);
                         active_keys_guard.insert(mapping.key_high.clone());
                    }
                } else if active_keys_guard.contains(&mapping.key_high) {
                     let _ = enigo.key(key, enigo::Direction::Release);
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
    state.set_mappings(mappings);
    Ok(())
}

#[tauri::command]
pub fn get_keyboard_mapping(state: tauri::State<std::sync::Arc<KeyboardService>>) -> Result<Vec<KeyMapping>, String> {
    Ok(state.mappings.lock().unwrap().clone())
}
