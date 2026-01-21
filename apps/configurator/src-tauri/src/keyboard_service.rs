use ffbeast_controller::models::wheel_status::WheelStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;
use enigo::{Enigo, Key, Keyboard, Settings, Direction};

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
    enigo: Mutex<Option<Enigo>>,
}

impl KeyboardService {
    pub fn new() -> Self {
        // Initialize Enigo. Returns None if backend initialization fails (e.g., missing X11 libs on Linux).
        // This prevents the application from crashing on systems without proper input support.
        let enigo = Enigo::new(&Settings::default()).ok();
        
        if enigo.is_none() {
            tracing::warn!("KeyboardService: Failed to initialize Enigo (missing dependencies?). Keyboard simulation will be disabled.");
        }

        Self {
            active: Mutex::new(false),
            mappings: Mutex::new(Vec::new()),
            active_keys: Mutex::new(HashSet::new()),
            enigo: Mutex::new(enigo),
        }
    }

    pub fn set_mappings(&self, new_mappings: Vec<KeyMapping>) {
        tracing::info!("KeyboardService: Setting {} mappings", new_mappings.len());
        let mut m = self.mappings.lock().unwrap();
        *m = new_mappings;
        self.active_keys.lock().unwrap().clear();
    }

    pub fn set_active(&self, active: bool) {
        tracing::info!("KeyboardService: Setting active = {}", active);
        *self.active.lock().unwrap() = active;
        if !active {
            // Release all currently held keys
            let mut active_state = self.active_keys.lock().unwrap();
            let mappings = self.mappings.lock().unwrap();
            let mut enigo_guard = self.enigo.lock().unwrap();
            
            if let Some(enigo) = enigo_guard.as_mut() {
                tracing::info!(
                    "KeyboardService: Releasing {} held keys",
                    active_state.len()
                );
                for id in active_state.iter() {
                    if let Some(map) = mappings.iter().find(|m| &m.id == id) {
                        if let Some(vk) = parse_key(&map.key) {
                            let _ = enigo.key(vk, Direction::Release);
                        }
                    }
                }
            }
            active_state.clear();
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

        // Debug log throttle
        static PROCESS_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let count = PROCESS_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if count % 120 == 0 {
             // Reduced log verbosity
        }

        let inputs_to_process = {
            let m = self.mappings.lock().unwrap();
            m.clone()
        };

        let mut active_state = self.active_keys.lock().unwrap();
        let mut enigo_guard = self.enigo.lock().unwrap();
        
        // If Enigo failed to init, we just skip processing but don't crash
        let enigo = match enigo_guard.as_mut() {
            Some(e) => e,
            None => return,
        };

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
                    tracing::info!("KeyboardService: Press {:?}", vk);
                    let _ = enigo.key(vk, Direction::Press);
                }
                active_state.insert(map.id.clone());
            } else if !is_triggered && was_triggered {
                if let Some(vk) = parse_key(&map.key) {
                    tracing::info!("KeyboardService: Release {:?}", vk);
                    let _ = enigo.key(vk, Direction::Release);
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
        "CAPSLOCK" => Some(Key::CapsLock),
        "META" | "SUPER" | "WIN" => Some(Key::Meta),
        "OPTION" => Some(Key::Option),
        // F keys
        "F1" => Some(Key::F1), "F2" => Some(Key::F2), "F3" => Some(Key::F3),
        "F4" => Some(Key::F4), "F5" => Some(Key::F5), "F6" => Some(Key::F6),
        "F7" => Some(Key::F7), "F8" => Some(Key::F8), "F9" => Some(Key::F9),
        "F10" => Some(Key::F10), "F11" => Some(Key::F11), "F12" => Some(Key::F12),
        // Chars
        c if c.len() == 1 => {
             let ch = c.chars().next().unwrap();
             // Force lowercase to ensure we send standard keys without implicity Shift
             Some(Key::Unicode(ch.to_ascii_lowercase())) 
        }
        _ => None
    }
}
