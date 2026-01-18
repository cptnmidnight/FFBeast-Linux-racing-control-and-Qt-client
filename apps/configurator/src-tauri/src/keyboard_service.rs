use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use ffbeast_controller::models::wheel_status::WheelStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyMapping {
    pub id: String,
    pub source_type: String, // "button", "axis"
    pub index: usize,
    pub trigger: String, // "press", "high", "low"
    pub key: String,     // "A", "SPACE", etc.
    pub threshold: Option<i32>,
}

pub struct KeyboardService {
    active: Mutex<bool>,
    mappings: Mutex<Vec<KeyMapping>>,
    active_keys: Mutex<HashSet<String>>, // Track triggered Mapping IDs
    enigo: Mutex<Enigo>,
}

impl KeyboardService {
    pub fn new() -> Self {
        Self {
            active: Mutex::new(false),
            mappings: Mutex::new(Vec::new()),
            active_keys: Mutex::new(HashSet::new()),
            enigo: Mutex::new(Enigo::new(&Settings::default()).expect("Failed to init Enigo")),
        }
    }

    pub fn set_mappings(&self, new_mappings: Vec<KeyMapping>) {
        let mut m = self.mappings.lock().unwrap();
        *m = new_mappings;
        // In a real scenario, we should release currently held keys if their mapping is removed.
        // For simplicity, we assume user stops service before reconfiguring.
        self.active_keys.lock().unwrap().clear();
    }

    pub fn set_active(&self, active: bool) {
        *self.active.lock().unwrap() = active;
        if !active {
            // Release all keys on stop?
            // To be safe, we could iterate active_keys and release.
            // But we need the Mapping object to know WHICH key.
            // For now, we leave it simple.
        }
    }

    pub fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }

    pub fn process(&self, status: &WheelStatus) {
        if !self.is_active() {
            return;
        }

        let inputs_to_process = {
            let m = self.mappings.lock().unwrap();
            m.clone()
        };

        let mut active_state = self.active_keys.lock().unwrap();
        let mut enigo = self.enigo.lock().unwrap();

        for map in inputs_to_process {
            let is_triggered = match map.source_type.as_str() {
                "button" => {
                    if map.index < 32 {
                        (status.buttons & (1 << map.index)) != 0
                    } else {
                        false
                    }
                }
                "axis" => {
                    let val = status.adc.get(map.index).cloned().unwrap_or(0);
                    let thr = map.threshold.unwrap_or(2048);
                    match map.trigger.as_str() {
                        "high" => (val as i32) > thr,
                        "low" => (val as i32) < thr,
                        _ => false,
                    }
                }
                _ => false,
            };

            let was_triggered = active_state.contains(&map.id);

            if is_triggered && !was_triggered {
                if let Some(k) = parse_key(&map.key) {
                    tracing::info!(
                        "KeyboardService: Triggering Key Press: {:?} (ID: {})",
                        k,
                        map.id
                    );
                    let _ = enigo.key(k, Direction::Press);
                }
                active_state.insert(map.id.clone());
            } else if !is_triggered && was_triggered {
                if let Some(k) = parse_key(&map.key) {
                    tracing::info!("KeyboardService: Releasing Key: {:?} (ID: {})", k, map.id);
                    let _ = enigo.key(k, Direction::Release);
                }
                active_state.remove(&map.id);
            }
        }
    }
}

fn parse_key(k: &str) -> Option<Key> {
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
        _ => {
            // Single char
            if k.len() == 1 {
                Some(Key::Unicode(k.chars().next().unwrap()))
            } else {
                None
            }
        }
    }
}
