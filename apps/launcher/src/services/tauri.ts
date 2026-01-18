import { invoke } from "@tauri-apps/api/core";
import { Game } from "../types";

export const GameService = {
    async getGames(): Promise<Game[]> {
        return await invoke("get_games");
    },

    async scanGames(): Promise<Game[]> {
        return await invoke("scan_games");
    },

    async saveGame(game: Game): Promise<void> {
        return await invoke("save_game", { game });
    },

    async launchGame(id: string): Promise<void> {
        return await invoke("launch_game", { id });
    }
};

export const HardwareService = {
    async checkConnection(): Promise<boolean> {
        return await invoke("check_hardware");
    },

    async resetCenter(): Promise<void> {
        return await invoke("reset_center");
    },

    async rebootDevice(): Promise<void> {
        return await invoke("reboot_device");
    }
};
