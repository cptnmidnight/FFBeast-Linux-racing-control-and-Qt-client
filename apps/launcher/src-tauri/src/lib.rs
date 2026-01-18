pub mod models;
pub mod storage;
pub mod services;

use crate::models::WheelProfile;

use crate::storage::{StorageBackend, TomlStorage};
use ffbeast_controller::{EffectSettings, HardwareService, HardwareSettings, WheelInterface};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tauri::Emitter as _;
use tracing::{info, instrument};
use tracing_subscriber::fmt::format::FmtSpan;

#[tauri::command]
#[instrument(skip(hardware))]
fn check_hardware(hardware: State<'_, Arc<HardwareService>>) -> bool {
    hardware.is_connected()
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn reset_center(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.send_reset_center().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn reboot_device(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.reboot_device().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn update_effect_settings(hardware: State<'_, Arc<HardwareService>>, settings: EffectSettings) -> Result<(), String> {
    hardware.send_effect_settings(settings).map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn update_hardware_settings(hardware: State<'_, Arc<HardwareService>>, settings: HardwareSettings) -> Result<(), String> {
    hardware.send_hardware_settings(settings).map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(handle), err)]
fn open_advanced_config(handle: tauri::AppHandle) -> Result<(), String> {
    let window = tauri::WebviewWindowBuilder::new(
        &handle,
        "advanced_config",
        tauri::WebviewUrl::App("advanced_config.html".into())
    )
    .title("SODevs Launcher - Configuração Avançada")
    .inner_size(900.0, 700.0)
    .resizable(true)
    .center()
    .build()
    .map_err(|e| e.to_string())?;
    
    // Open DevTools in development mode
    #[cfg(debug_assertions)]
    {
        window.open_devtools();
    }
    
    Ok(())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn save_settings_to_hardware(hardware: State<'_, Arc<HardwareService>>) -> Result<(), String> {
    hardware.save_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn get_effect_settings(hardware: State<'_, Arc<HardwareService>>) -> Result<EffectSettings, String> {
    hardware.read_effect_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(hardware), err)]
fn get_hardware_settings(hardware: State<'_, Arc<HardwareService>>) -> Result<HardwareSettings, String> {
    hardware.read_hardware_settings().map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(storage))]
fn get_games(storage: State<'_, Arc<TomlStorage>>) -> Vec<crate::models::Game> {
    storage.list_games().unwrap_or_default()
}

#[tauri::command]
#[instrument]
async fn scan_games() -> Vec<crate::models::Game> {
    crate::services::game_scanner::GameScanner::scan()
}

#[tauri::command]
#[instrument(skip(storage), err)]
fn save_game(storage: State<'_, Arc<TomlStorage>>, game: crate::models::Game) -> Result<(), String> {
    storage.save_game(game).map_err(|e| e.to_string())
}

#[tauri::command]
#[instrument(skip(storage, hardware), fields(game_id = %id), err)]
async fn launch_game(
    id: String,
    storage: State<'_, Arc<TomlStorage>>,
    hardware: State<'_, Arc<HardwareService>>,
) -> Result<(), String> {
    let games = storage.list_games().map_err(|e| e.to_string())?;
    
    // Try to find in storage first
    let game = match games.into_iter().find(|g| g.id == id) {
        Some(g) => g,
        None => {
            // If not in storage, check scanner
            let scanned = crate::services::game_scanner::GameScanner::scan();
            scanned.into_iter().find(|g| g.id == id).ok_or("Game not found in library or scanner")?
        }
    };
    
    // Convert WheelProfileSettings to WheelProfile if present
    let profile = game.wheel_profile.as_ref().map(|settings| {
        WheelProfile {
            id: String::new(), // Not needed for runtime
            name: String::new(), // Not needed for runtime
            motion_range: settings.motion_range,
            total_force: settings.total_force,
            dynamic_dampening: 50, // Default values
            static_dampening: 50,
            power_limit: 100,
            braking_limit: 100,
        }
    });

    let default_profile = WheelProfile::default();
    
    let child = crate::services::GameRunner::run(&game, profile.as_ref())
        .map_err(|e| e.to_string())?;
        
    crate::services::ProfileWatcher::watch(
        child, 
        profile, 
        hardware.inner().clone(), 
        default_profile
    );

    Ok(())
}

#[tauri::command]
#[instrument]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to SODevs Game Launcher!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with_span_events(FmtSpan::CLOSE)
        .init();

    info!("Starting SODevs Launcher");
    
    // Fix for Linux WebKit rendering issue (Blank screen)
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    let hardware = Arc::new(HardwareService::new());
    let hardware_clone = hardware.clone();
    
    let storage = Arc::new(TomlStorage::new(PathBuf::from("launcher_settings.toml")).unwrap());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(hardware)
        .manage(storage)
        .setup(move |app| {
            let handle = app.handle().clone();
            
            std::thread::spawn(move || {
                loop {
                    if !hardware_clone.is_connected() {
                        let _ = hardware_clone.connect();
                    }

                    if let Ok(status) = hardware_clone.read_status() {
                        let _ = handle.emit("wheel-status", status);
                    }
                    
                    std::thread::sleep(std::time::Duration::from_millis(16));
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet, 
            check_hardware, 
            get_games,
            scan_games,
            save_game,
            reset_center,
            reboot_device,
            update_effect_settings,
            update_hardware_settings,
            get_effect_settings,
            get_hardware_settings,
            save_settings_to_hardware,
            open_advanced_config,
            launch_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
