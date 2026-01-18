use crate::models::{Game, WheelProfile};
use anyhow::Result;

pub trait StorageBackend {
    // Games
    fn list_games(&self) -> Result<Vec<Game>>;
    fn save_game(&self, game: Game) -> Result<()>;
    fn delete_game(&self, id: &str) -> Result<()>;
    
    // Profiles
    fn list_profiles(&self) -> Result<Vec<WheelProfile>>;
    fn save_profile(&self, profile: WheelProfile) -> Result<()>;
    fn get_profile(&self, id: &str) -> Result<Option<WheelProfile>>;
}
