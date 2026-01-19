use crate::models::{Game, WheelProfile};
use anyhow::{anyhow, Result};
use std::process::{Child, Command};
use tracing::{info, instrument};

pub struct GameRunner;

impl GameRunner {
    #[instrument(skip(game, profile), fields(game = %game.name), err)]
    pub fn run(game: &Game, profile: Option<&WheelProfile>) -> Result<Child> {
        // 1. Preparar o comando base
        let mut cmd = if cfg!(target_os = "linux") && game.use_compat_layer {
            Self::prepare_proton_command(game)
        } else {
            Self::prepare_native_command(game)
        };

        // 2. Injetar Variáveis de Ambiente
        for (key, value) in &game.environment_vars {
            cmd.env(key, value);
        }

        // 3. Injetar DLL Overrides (Específico para Wine/Proton no Linux)
        if !game.dll_overrides.is_empty() {
            let overrides = game.dll_overrides.join(";");
            cmd.env("WINEDLLOVERRIDES", overrides);
        }

        // 4. Aplicar Perfil do Volante (Placeholder para integração HID)
        if let Some(p) = profile {
            info!(
                "Aplicando perfil de volante: {} para o jogo {}",
                p.name, game.name
            );
            // Aqui chamaremos o ffbeast-core no futuro
        }

        // 5. Executar
        info!("Lançando jogo: {} em {:?}", game.name, game.path);
        cmd.spawn()
            .map_err(|e| anyhow!("Falha ao lançar o jogo: {}", e))
    }

    fn prepare_native_command(game: &Game) -> Command {
        let mut cmd = Command::new(&game.path);
        cmd.args(&game.arguments);

        // Configura o diretório de trabalho para a pasta do jogo
        if let Some(parent) = game.path.parent() {
            cmd.current_dir(parent);
        }

        cmd
    }

    fn prepare_proton_command(game: &Game) -> Command {
        // No Linux, idealmente buscaríamos o caminho do Proton via config ou detecção automática
        // Por agora, assumimos 'proton run' via shell ou caminho direto configurado
        let mut cmd = Command::new("proton");
        cmd.arg("run");
        cmd.arg(&game.path);
        cmd.args(&game.arguments);

        if let Some(parent) = game.path.parent() {
            cmd.current_dir(parent);
        }

        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_native_command_args() {
        let game = Game {
            id: "test".to_string(),
            name: "Test Game".to_string(),
            path: PathBuf::from("cmd.exe"),
            arguments: vec!["/c".to_string(), "echo".to_string(), "hello".to_string()],
            ..Default::default()
        };

        let cmd = GameRunner::prepare_native_command(&game);
        assert_eq!(cmd.get_program(), "cmd.exe");
        let args: Vec<_> = cmd.get_args().map(|s| s.to_str().unwrap()).collect();
        assert_eq!(args, vec!["/c", "echo", "hello"]);
    }
}
