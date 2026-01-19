import { invoke } from '@tauri-apps/api/core';
import type { HardwareStatus } from '../models/HardwareStatus';
import type { EffectSettings } from '../models/EffectSettings';
import type { HardwareSettings } from '../models/HardwareSettings';
import type { GpioSettings } from '../models/GpioSettings';

export const HardwareService = {
    async getHandshake(): Promise<{ status: HardwareStatus, fx: EffectSettings, hw: HardwareSettings, gpio: GpioSettings }> {
        return await invoke('get_handshake');
    },

    async pollStatus(): Promise<HardwareStatus> {
        return await invoke('get_status');
    },

    async updateEffectSettings(settings: EffectSettings): Promise<void> {
        await invoke('save_effect_settings', { settings });
    },

    async updateHardwareSettings(settings: HardwareSettings): Promise<void> {
        await invoke('save_hardware_settings', { settings });
    },

    async updateGpioSettings(settings: GpioSettings): Promise<void> {
        await invoke('save_gpio_settings', { settings });
    },

    async reboot(): Promise<void> {
        await invoke('reboot_device');
    },

    async resetCenter(): Promise<void> {
        await invoke('reset_center');
    },

    async saveToEeprom(): Promise<void> {
        await invoke('save_all_to_eeprom');
    }
};
