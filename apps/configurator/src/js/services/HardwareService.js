const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

export const HardwareService = {
    async isConnected() {
        return await invoke('check_hardware');
    },

    async connect() {
        return await invoke('connect_hardware');
    },

    async reboot() {
        return await invoke('reboot_device');
    },

    async resetCenter() {
        return await invoke('reset_center');
    },

    async save() {
        return await invoke('save_settings');
    },

    async getEffectSettings() {
        return await invoke('get_effect_settings');
    },

    async getHardwareSettings() {
        return await invoke('get_hardware_settings');
    },

    async getGpioSettings() {
        return await invoke('get_gpio_settings');
    },

    async getAdcSettings() {
        return await invoke('get_adc_settings');
    },

    async updateEffectSettings(settings) {
        return await invoke('update_effect_settings', { settings });
    },

    async updateHardwareSettings(settings) {
        return await invoke('update_hardware_settings', { settings });
    },

    async updateGpioSettings(settings) {
        return await invoke('update_gpio_settings', { settings });
    },

    async updateAdcSettings(settings) {
        return await invoke('update_adc_settings', { settings });
    },

    async activateLicense(keyStr) {
        return await invoke('activate_license', { keyStr });
    },

    async switchToDfu() {
        return await invoke('switch_to_dfu');
    },

    async sendDirectControl(forceType, value) {
        return await invoke('send_direct_control', { forceType, value });
    },

    async toggleKeyboardService(enabled) {
        return await invoke('toggle_keyboard_service', { enabled });
    },

    async onStatusUpdate(callback) {
        return await listen('wheel-status', (event) => {
            if (event.payload) callback(event.payload);
        });
    }
};
