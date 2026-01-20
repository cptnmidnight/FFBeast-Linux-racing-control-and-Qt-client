use crate::native_keyboard::NativeKeyboard;
use crate::virtual_key::parse_key;
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

    pub fn set_mappings(&self, new_mappings: Vec<KeyMapping>) {
        tracing::info!("KeyboardService: Setting {} mappings", new_mappings.len());
        for (i, mapping) in new_mappings.iter().enumerate() {
            tracing::info!(
                "  Mapping {}: {} {} -> {} (trigger: {})",
                i,
                mapping.source_type,
                mapping.index,
                mapping.key,
                mapping.trigger
            );
        }
        let mut m = self.mappings.lock().unwrap();
        *m = new_mappings;
        self.active_keys.lock().unwrap().clear();
    }

    pub fn set_active(&self, active: bool) {
        tracing::info!("KeyboardService: Setting active = {}", active);
        *self.active.lock().unwrap() = active;
        if !active {
            // Release all currently held keys
            let active_state = self.active_keys.lock().unwrap();
            let mappings = self.mappings.lock().unwrap();

            tracing::info!(
                "KeyboardService: Releasing {} held keys",
                active_state.len()
            );
            for id in active_state.iter() {
                if let Some(map) = mappings.iter().find(|m| &m.id == id) {
                    if let Some(vk) = parse_key(&map.key) {
                        let _ = NativeKeyboard::send_key(vk, false);
                    }
                }
            }
        } else {
            let count = self.mappings.lock().unwrap().len();
            tracing::info!("KeyboardService: Service activated with {} mappings", count);
        }
    }

    pub fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }

    pub fn process(&self, status: &WheelStatus) {
        if !self.is_active() {
            return;
        }

        // Debug: Log every ~1 second to confirm we're processing
        static PROCESS_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let count = PROCESS_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if count % 120 == 0 {
            tracing::info!(
                "KeyboardService: Processing - buttons: 0x{:08X}, axes: [{}, {}, {}, {}, {}, {}]",
                status.buttons,
                status.adc.get(0).unwrap_or(&0),
                status.adc.get(1).unwrap_or(&0),
                status.adc.get(2).unwrap_or(&0),
                status.adc.get(3).unwrap_or(&0),
                status.adc.get(4).unwrap_or(&0),
                status.adc.get(5).unwrap_or(&0)
            );
        }

        let inputs_to_process = {
            let m = self.mappings.lock().unwrap();
            m.clone()
        };

        let mut active_state = self.active_keys.lock().unwrap();

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
                if let Some(vk) = parse_key(&map.key) {
                    tracing::info!(
                        "KeyboardService: Triggering Key Press: {:?} (ID: {}, source: {} {})",
                        vk,
                        map.id,
                        map.source_type,
                        map.index
                    );
                    match NativeKeyboard::send_key(vk, true) {
                        Ok(_) => tracing::info!("  -> Key press successful"),
                        Err(e) => tracing::error!("  -> Key press FAILED: {}", e),
                    }
                }
                active_state.insert(map.id.clone());
            } else if !is_triggered && was_triggered {
                if let Some(vk) = parse_key(&map.key) {
                    tracing::info!("KeyboardService: Releasing Key: {:?} (ID: {})", vk, map.id);
                    match NativeKeyboard::send_key(vk, false) {
                        Ok(_) => {}
                        Err(e) => tracing::error!("  -> Key release FAILED: {}", e),
                    }
                }
                active_state.remove(&map.id);
            }
        }
    }
}
