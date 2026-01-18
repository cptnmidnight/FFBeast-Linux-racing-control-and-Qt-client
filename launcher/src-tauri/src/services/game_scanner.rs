use crate::models::Game;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

pub struct GameScanner;

impl GameScanner {
    pub fn scan() -> Vec<Game> {
        let mut games = Vec::new();
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

        // Common Steam Library paths on Linux
        let steam_paths = vec![
            PathBuf::from(&home).join(".steam/steam/steamapps/common"),
            PathBuf::from(&home).join(".local/share/Steam/steamapps/common"),
        ];

        info!("Scanning for games in: {:?}", steam_paths);

        for library_path in steam_paths {
            if !library_path.exists() {
                continue;
            }

            // Assetto Corsa
            let ac_path = library_path.join("assettocorsa/acs.exe");
            if ac_path.exists() {
                info!("Found Assetto Corsa at {:?}", ac_path);
                games.push(Game {
                    id: "assetto_corsa".to_string(),
                    name: "Assetto Corsa".to_string(),
                    path: ac_path,
                    arguments: vec![],
                    environment_vars: Default::default(),
                    dll_overrides: vec![],
                    wheel_profile: None,
                    use_compat_layer: true,
                    icon_path: None,
                    cover_path: None,
                });
            }

            // Assetto Corsa Competizione
            let acc_path = library_path.join("Assetto Corsa Competizione/AC2.exe");
            if acc_path.exists() {
                info!("Found Assetto Corsa Competizione at {:?}", acc_path);
                games.push(Game {
                    id: "assetto_corsa_competizione".to_string(),
                    name: "Assetto Corsa Competizione".to_string(),
                    path: acc_path,
                    arguments: vec![],
                    environment_vars: Default::default(),
                    dll_overrides: vec![],
                    wheel_profile: None,
                    use_compat_layer: true,
                    icon_path: None,
                    cover_path: None,
                });
            }

            // Automobilista 2
            let ams2_path = library_path.join("Automobilista 2/AMS2.exe");
            if ams2_path.exists() {
                info!("Found Automobilista 2 at {:?}", ams2_path);
                games.push(Game {
                    id: "automobilista_2".to_string(),
                    name: "Automobilista 2".to_string(),
                    path: ams2_path,
                    arguments: vec![],
                    environment_vars: Default::default(),
                    dll_overrides: vec![],
                    wheel_profile: None,
                    use_compat_layer: true,
                    icon_path: None,
                    cover_path: None,
                });
            }

            // BeamNG.drive
            let beamng_path = library_path.join("BeamNG.drive/Bin64/BeamNG.drive.x64.exe");
            if beamng_path.exists() {
                 info!("Found BeamNG.drive at {:?}", beamng_path);
                 games.push(Game {
                    id: "beamng_drive".to_string(),
                    name: "BeamNG.drive".to_string(),
                    path: beamng_path,
                    arguments: vec![],
                    environment_vars: Default::default(),
                    dll_overrides: vec![],
                    wheel_profile: None,
                    use_compat_layer: true,
                    icon_path: None,
                    cover_path: None,
                });
            }
        }
        
        games
    }
}
