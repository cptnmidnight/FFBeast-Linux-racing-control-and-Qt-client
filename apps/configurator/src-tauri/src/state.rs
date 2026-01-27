use std::sync::OnceLock;
use std::sync::atomic::AtomicU8;

pub static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();
pub static MIN_LOG_LEVEL: AtomicU8 = AtomicU8::new(3); // Default to INFO (3)
