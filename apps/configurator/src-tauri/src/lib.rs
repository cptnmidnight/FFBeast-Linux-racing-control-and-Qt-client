use ffbeast_controller::{
    AdcSettings, EffectSettings, GpioSettings, HardwareService, HardwareSettings, WheelInterface,
};
use std::sync::Arc;
use tauri::{Emitter, State};
use tracing::{info, instrument};

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    info!("Starting FFBeast UI");

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

                    std::thread::sleep(std::time::Duration::from_millis(16));
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_hardware,
            connect_hardware,
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
            send_direct_control
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
