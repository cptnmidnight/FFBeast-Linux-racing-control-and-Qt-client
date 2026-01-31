mod logging;
mod models;
mod state;
mod commands;

use ffbeast_controller::{HardwareService, WheelInterface};
use sodevs_input_manager::InputManager;
use std::sync::Arc;
use tauri::Emitter;
use tracing::info;
use tracing_subscriber::prelude::*;

pub use state::{APP_HANDLE, MIN_LOG_LEVEL};
pub use logging::{GlobalLevelFilter, TauriLogLayer};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize tracing FIRST, before any other code that might log
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "info,ffbeast_controller=trace,ffbeast_ui_lib=trace,windows_gamepad=trace".into()
    });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_filter(GlobalLevelFilter))
        .with(TauriLogLayer.with_filter(GlobalLevelFilter))
        .init();

    info!("Starting FFBeast UI");

    // Fix for Linux WebKit rendering issue (Blank screen)
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    let hardware = Arc::new(HardwareService::new());
    let keyboard = Arc::new(InputManager::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(hardware.clone())
        .manage(keyboard.clone())
        .setup(move |app| {
            // Store app handle for log forwarding
            let _ = APP_HANDLE.set(app.handle().clone());

            let hw = hardware.clone();
            let kb = keyboard.clone();
            let handle = app.handle().clone();

            std::thread::spawn(move || {
                let mut last_read_failed = false;
                loop {
                    if hw.is_connected() {
                        match hw.read_status() {
                            Ok(status) => {
                                kb.process(&status);
                                let _ = handle.emit("wheel-status", &status);
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
            commands::check_hardware,
            commands::connect_hardware,
            commands::get_handshake,
            commands::get_status,
            commands::reboot_device,
            commands::reset_center,
            commands::save_settings,
            commands::get_effect_settings,
            commands::get_hardware_settings,
            commands::get_gpio_settings,
            commands::get_adc_settings,
            commands::update_effect_settings,
            commands::update_hardware_setting,
            commands::update_gpio_settings,
            commands::update_adc_settings,
            commands::activate_license,
            commands::set_keyboard_service_active,
            commands::get_keyboard_service_active,
            commands::set_keyboard_mapping,
            commands::get_keyboard_config,
            commands::set_keyboard_config,
            commands::switch_to_dfu,
            commands::send_direct_control,
            commands::get_versions,
            commands::set_min_log_level
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
