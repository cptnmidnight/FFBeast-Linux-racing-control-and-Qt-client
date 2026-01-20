use ffbeast_controller::{
    AdcSettings, EffectSettings, GpioSettings, HardwareService, HardwareSettings, WheelInterface,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use tauri::{Emitter, State};
use tracing::{info, instrument};
use tracing_subscriber::prelude::*;

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();
static MIN_LOG_LEVEL: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(3); // Default to INFO (3)

struct LogVisitor {
    message: String,
}

impl LogVisitor {
    fn new() -> Self {
        Self {
            message: String::new(),
        }
    }
}

impl tracing::field::Visit for LogVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }
}

struct TauriLogLayer;

impl<S> tracing_subscriber::Layer<S> for TauriLogLayer
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let metadata = event.metadata();
        let level = metadata.level();

        // Level mapping: Error=1, Warn=2, Info=3, Debug=4, Trace=5
        // We want to filter out logs more verbose than our setting.
        // If MIN_LOG_LEVEL is 3 (Info), we only want 1, 2, 3.
        let level_num = match *level {
            tracing::Level::ERROR => 1,
            tracing::Level::WARN => 2,
            tracing::Level::INFO => 3,
            tracing::Level::DEBUG => 4,
            tracing::Level::TRACE => 5,
        };

        let min_level = MIN_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed);
        if level_num > min_level {
            return;
        }

        if let Some(handle) = APP_HANDLE.get() {
            let level_str = level.to_string().to_lowercase();
            let mut visitor = LogVisitor::new();
            event.record(&mut visitor);

            if !visitor.message.is_empty() {
                let _ = handle.emit(
                    "rust-log",
                    serde_json::json!({
                        "level": level_str,
                        "message": visitor.message,
                    }),
                );
            }
        }
    }
}

mod keyboard_service;
use keyboard_service::{KeyMapping, KeyboardService};

#[tauri::command]
fn check_hardware(hardware: State<'_, Arc<HardwareService>>) -> bool {
    let connected = hardware.is_connected();
    connected
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn connect_hardware(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.connect().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn reboot_device(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.reboot_device().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn reset_center(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.send_reset_center().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn save_settings(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.save_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn get_effect_settings(
    hardware: State<'_, Arc<HardwareService>>,
) -> Result<EffectSettings, String> {
    hardware.read_effect_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn get_hardware_settings(
    hardware: State<'_, Arc<HardwareService>>,
) -> Result<HardwareSettings, String> {
    hardware.read_hardware_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn get_gpio_settings(hardware: State<'_, Arc<HardwareService>>) -> Result<GpioSettings, String> {
    hardware.read_gpio_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn get_adc_settings(hardware: State<'_, Arc<HardwareService>>) -> Result<AdcSettings, String> {
    hardware.read_adc_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn update_effect_settings(
    hardware: State<'_, Arc<HardwareService>>,
    settings: EffectSettings,
) -> Result<(), String> {
    hardware
        .send_effect_settings(settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn update_hardware_settings(
    hardware: State<'_, Arc<HardwareService>>,
    settings: HardwareSettings,
) -> Result<(), String> {
    hardware
        .send_hardware_settings(settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn update_gpio_settings(
    hardware: State<'_, Arc<HardwareService>>,
    settings: GpioSettings,
) -> Result<(), String> {
    hardware
        .send_gpio_settings(settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn update_adc_settings(
    hardware: State<'_, Arc<HardwareService>>,
    settings: AdcSettings,
) -> Result<(), String> {
    hardware
        .send_adc_settings(settings)
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct HandshakeResponse {
    status: ffbeast_controller::WheelStatus,
    fx: EffectSettings,
    hw: HardwareSettings,
    gpio: GpioSettings,
    adc: AdcSettings,
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn get_handshake(hardware: State<'_, Arc<HardwareService>>) -> Result<HandshakeResponse, String> {
    // Try to connect if not already connected
    if !hardware.is_connected() {
        hardware.connect().map_err(|e| e.to_string())?;
    }

    let status = hardware.read_status().map_err(|e| e.to_string())?;
    let fx = hardware.read_effect_settings().map_err(|e| e.to_string())?;
    let hw = hardware
        .read_hardware_settings()
        .map_err(|e| e.to_string())?;
    let gpio = hardware.read_gpio_settings().map_err(|e| e.to_string())?;
    let adc = hardware.read_adc_settings().map_err(|e| e.to_string())?;

    Ok(HandshakeResponse {
        status,
        fx,
        hw,
        gpio,
        adc,
    })
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn get_status(
    hardware: State<'_, Arc<HardwareService>>,
) -> Result<ffbeast_controller::WheelStatus, String> {
    hardware.read_status().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn activate_license(
    hardware: State<'_, Arc<HardwareService>>,
    key_str: String,
) -> Result<(), String> {
    let clean = key_str.trim().replace("-", "").replace(" ", "");
    // Expect 24 hex chars (3x 32-bit = 96 bits / 4 = 24 hex)
    if clean.len() != 24 {
        return Err("Invalid key length. Expected 24 hex characters.".into());
    }

    let mut key = [0u32; 3];
    for i in 0..3 {
        let chunk = &clean[i * 8..(i + 1) * 8];
        match u32::from_str_radix(chunk, 16) {
            Ok(v) => key[i] = v,
            Err(e) => return Err(format!("Invalid hex: {}", e)),
        }
    }

    hardware.activate_license(key).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn switch_to_dfu(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.switch_to_dfu().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn send_direct_control(
    hardware: State<'_, Arc<HardwareService>>,
    force_type: u8,
    value: i16,
) -> Result<(), String> {
    hardware
        .send_direct_control(force_type, value)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_keyboard_service(
    service: State<'_, Arc<KeyboardService>>,
    enabled: bool,
) -> Result<(), String> {
    service.set_active(enabled);
    Ok(())
}

#[tauri::command]
fn set_keyboard_mapping(
    service: State<'_, Arc<KeyboardService>>,
    mappings: Vec<KeyMapping>,
) -> Result<(), String> {
    service.set_mappings(mappings);
    Ok(())
}

#[derive(serde::Serialize)]
struct AppVersions {
    app: String,
    controller: String,
}

#[tauri::command]
fn get_versions() -> AppVersions {
    AppVersions {
        app: env!("CARGO_PKG_VERSION").to_string(),
        controller: ffbeast_controller::VERSION.to_string(),
    }
}

#[tauri::command]
fn set_min_log_level(level: u8) {
    info!("Minimum log level set to: {}", level);
    MIN_LOG_LEVEL.store(level, std::sync::atomic::Ordering::Relaxed);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,ffbeast_controller=debug,ffbeast_ui_lib=debug".into());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .with(TauriLogLayer)
        .init();

    info!("Starting FFBeast UI");

    // Fix for Linux WebKit rendering issue (Blank screen)
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    let hardware = Arc::new(HardwareService::new());
    let hardware_clone = hardware.clone();

    let kb_service = Arc::new(KeyboardService::new());
    let kb_service_clone = kb_service.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(hardware)
        .manage(kb_service)
        .setup(move |app| {
            let handle = app.handle().clone();
            let _ = APP_HANDLE.set(handle.clone());

            std::thread::spawn(move || {
                let mut last_read_failed = false;
                loop {
                    if hardware_clone.is_connected() {
                        match hardware_clone.read_status() {
                            Ok(status) => {
                                if status.adc.iter().any(|&v| v > 0) {
                                    // tracing::debug!("Telemetry ADC active: {:?}", status.adc);
                                }
                                let _ = handle.emit("wheel-status", &status);
                                kb_service_clone.process(&status);
                                if last_read_failed {
                                    info!("Telemetry resumed successfully.");
                                    last_read_failed = false;
                                }
                            }
                            Err(e) => {
                                if !last_read_failed {
                                    tracing::warn!(
                                        "Failed to read status: {}. Telemetry paused.",
                                        e
                                    );
                                    last_read_failed = true;
                                }
                            }
                        }
                    } else {
                        last_read_failed = false;
                    }

                    // Poll at ~120Hz for smoother wheel updates
                    std::thread::sleep(std::time::Duration::from_millis(8));
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_hardware,
            connect_hardware,
            get_handshake,
            get_status,
            reboot_device,
            reset_center,
            save_settings,
            get_effect_settings,
            get_hardware_settings,
            get_gpio_settings,
            get_adc_settings,
            update_effect_settings,
            update_hardware_settings,
            update_gpio_settings,
            update_adc_settings,
            activate_license,
            toggle_keyboard_service,
            set_keyboard_mapping,
            switch_to_dfu,
            send_direct_control,
            get_versions,
            set_min_log_level
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
