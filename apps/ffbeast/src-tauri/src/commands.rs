use std::sync::Arc;
use tauri::State;
use tracing::{instrument, info};
use ffbeast_controller::{
    AdcSettings, EffectSettings, GpioSettings, HardwareService, HardwareSettingId,
    HardwareSettings, WheelInterface, WheelStatus,
};
use sodevs_input_manager::{InputManager, KeyMapping, ManagerConfig};
use crate::models::{HandshakeResponse, AppVersions};
use crate::state::MIN_LOG_LEVEL;
use std::sync::atomic::Ordering;

#[tauri::command]
pub fn check_hardware(hardware: State<'_, Arc<HardwareService>>) -> bool {
    hardware.is_connected()
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn connect_hardware(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.connect().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn reboot_device(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.reboot_device().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn reset_center(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.send_reset_center().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn save_settings(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.save_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn get_effect_settings(
    hardware: State<'_, Arc<HardwareService>>,
) -> Result<EffectSettings, String> {
    hardware.read_effect_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn get_hardware_settings(
    hardware: State<'_, Arc<HardwareService>>,
) -> Result<HardwareSettings, String> {
    hardware.read_hardware_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn get_gpio_settings(hardware: State<'_, Arc<HardwareService>>) -> Result<GpioSettings, String> {
    hardware.read_gpio_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn get_adc_settings(hardware: State<'_, Arc<HardwareService>>) -> Result<AdcSettings, String> {
    hardware.read_adc_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn update_effect_settings(
    hardware: State<'_, Arc<HardwareService>>,
    settings: EffectSettings,
) -> Result<(), String> {
    hardware
        .send_effect_settings(settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn update_hardware_setting(
    hardware: State<'_, Arc<HardwareService>>,
    field_id: u8,
    index: u8,
    data: Vec<u8>,
) -> Result<(), String> {
    hardware
        .update_hardware_setting(HardwareSettingId::from(field_id), index, data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn update_gpio_settings(
    hardware: State<'_, Arc<HardwareService>>,
    settings: GpioSettings,
) -> Result<(), String> {
    hardware
        .send_gpio_settings(settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn update_adc_settings(
    hardware: State<'_, Arc<HardwareService>>,
    settings: AdcSettings,
) -> Result<(), String> {
    hardware
        .send_adc_settings(settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn get_handshake(hardware: State<'_, Arc<HardwareService>>) -> Result<HandshakeResponse, String> {
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
pub fn get_status(
    hardware: State<'_, Arc<HardwareService>>,
) -> Result<WheelStatus, String> {
    hardware.read_status().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn activate_license(
    hardware: State<'_, Arc<HardwareService>>,
    key_str: String,
) -> Result<(), String> {
    let clean = key_str.trim().replace("-", "").replace(" ", "");
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
pub fn switch_to_dfu(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.switch_to_dfu().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
pub fn send_direct_control(
    hardware: State<'_, Arc<HardwareService>>,
    force_type: u8,
    value: i16,
) -> Result<(), String> {
    hardware
        .send_direct_control(force_type, value)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_keyboard_service_active(service: State<'_, Arc<InputManager>>) -> bool {
    service.is_active()
}

#[tauri::command]
pub fn set_keyboard_service_active(
    service: State<'_, Arc<InputManager>>,
    hw: State<'_, Arc<HardwareService>>,
    enabled: bool,
) -> Result<(), String> {
    if enabled && !hw.is_connected() {
        return Err("Cannot start mapping service: Hardware not connected".to_string());
    }
    service.set_active(enabled);
    Ok(())
}

#[tauri::command]
pub fn set_keyboard_mapping(
    service: State<'_, Arc<InputManager>>,
    mappings: Vec<KeyMapping>,
) -> Result<(), String> {
    service.set_mappings(mappings);
    Ok(())
}

#[tauri::command]
pub fn get_keyboard_config(service: State<'_, Arc<InputManager>>) -> ManagerConfig {
    service.get_config()
}

#[tauri::command]
pub fn set_keyboard_config(
    service: State<'_, Arc<InputManager>>,
    config: ManagerConfig,
) -> Result<(), String> {
    service.set_config(config);
    Ok(())
}

#[tauri::command]
pub fn get_versions() -> AppVersions {
    AppVersions {
        app: env!("CARGO_PKG_VERSION").to_string(),
        controller: ffbeast_controller::VERSION.to_string(),
    }
}

#[tauri::command]
pub fn set_min_log_level(level: u8) {
    info!("Minimum log level set to: {}", level);
    MIN_LOG_LEVEL.store(level, Ordering::Relaxed);
}
