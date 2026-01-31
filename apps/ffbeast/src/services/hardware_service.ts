import { invoke } from '@tauri-apps/api/core';
import type { HardwareStatus } from '../models/HardwareStatus';
import type { EffectSettings } from '../models/EffectSettings';
import type { HardwareSettings } from '../models/HardwareSettings';
import type { GpioSettings } from '../models/GpioSettings';
import type { AdcSettings } from '../models/AdcSettings';
import type { KeyMapping } from '@shared/models/KeyMapping';
import type { KeyboardManagerConfig } from '@shared/models/KeyboardManagerConfig';
import { HardwareSettingId } from '../models/HardwareSettingId';

export const HardwareService = {
    async getHandshake(): Promise<{
        status: HardwareStatus,
        fx: EffectSettings,
        hw: HardwareSettings,
        gpio: GpioSettings,
        adc: AdcSettings
    }> {
        return await invoke('get_handshake');
    },

    async pollStatus(): Promise<HardwareStatus> {
        return await invoke('get_status');
    },

    async updateEffectSettings(settings: EffectSettings): Promise<void> {
        await invoke('update_effect_settings', { settings });
    },

    async updateHardwareSetting(fieldId: HardwareSettingId, index: number, data: number[]): Promise<void> {
        await invoke('update_hardware_setting', { fieldId, index, data });
    },

    async updateGpioSettings(settings: GpioSettings): Promise<void> {
        await invoke('update_gpio_settings', { settings });
    },

    async updateAdcSettings(settings: AdcSettings): Promise<void> {
        await invoke('update_adc_settings', { settings });
    },

    async reboot(): Promise<void> {
        await invoke('reboot_device');
    },

    async resetCenter(): Promise<void> {
        await invoke('reset_center');
    },

    async saveToEeprom(): Promise<void> {
        await invoke('save_settings');
    },

    async sendDirectControl(forceType: number, value: number): Promise<void> {
        await invoke('send_direct_control', { forceType, value: Math.round(value) });
    },

    async switchToDfu(): Promise<void> {
        await invoke('switch_to_dfu');
    },

    async activateLicense(key: string): Promise<void> {
        await invoke('activate_license', { keyStr: key });
    },

    async toggleKeyboardService(enabled: boolean): Promise<void> {
        await invoke('set_keyboard_service_active', { enabled });
    },

    async setKeyboardMapping(mappings: KeyMapping[]): Promise<void> {
        await invoke('set_keyboard_mapping', { mappings });
    },

    async setKeyboardServiceActive(enabled: boolean): Promise<void> {
        await invoke('set_keyboard_service_active', { enabled });
    },

    async getKeyboardServiceActive(): Promise<boolean> {
        return await invoke('get_keyboard_service_active');
    },

    async getKeyboardConfig(): Promise<KeyboardManagerConfig> {
        return await invoke('get_keyboard_config');
    },

    async setKeyboardConfig(config: KeyboardManagerConfig): Promise<void> {
        await invoke('set_keyboard_config', { config });
    },

    async setMinLogLevel(level: number): Promise<void> {
        return await invoke('set_min_log_level', { level });
    },

    async getVersions(): Promise<{ app: string, controller: string }> {
        return await invoke('get_versions');
    }
};
