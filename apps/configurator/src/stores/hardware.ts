import { defineStore } from 'pinia';
import { HardwareService } from '../services/hardware_service';
import type { HardwareStatus } from '../models/HardwareStatus';
import type { EffectSettings } from '../models/EffectSettings';
import type { HardwareSettings } from '../models/HardwareSettings';
import type { GpioSettings } from '../models/GpioSettings';

import type { AdcSettings } from '../models/AdcSettings';

export const useHardwareStore = defineStore('hardware', {
    state: () => ({
        isConnected: false,
        isConnecting: false,
        status: null as HardwareStatus | null,
        effects: null as EffectSettings | null,
        hardware: null as HardwareSettings | null,
        gpio: null as GpioSettings | null,
        adc: null as AdcSettings | null,
        lastError: null as string | null,
        reconnectTimer: null as ReturnType<typeof setInterval> | null,
    }),

    actions: {
        async init() {
            console.log('[HardwareStore] Initializing...');
            await this.connect();
            this.startAutoReconnect();
        },

        async connect() {
            if (this.isConnecting || this.isConnected) {
                console.log('[HardwareStore] Already connecting or connected, skipping');
                return;
            }

            console.log('[HardwareStore] Attempting to connect to hardware...');
            this.isConnecting = true;

            try {
                console.log('[HardwareStore] Calling getHandshake...');
                const data = await HardwareService.getHandshake();
                console.log('[HardwareStore] Handshake successful:', data);

                this.status = data.status;
                this.effects = data.fx;
                this.hardware = data.hw;
                this.gpio = data.gpio;
                this.adc = data.adc;
                this.isConnected = true;
                this.lastError = null;

                console.log('[HardwareStore] Connection established, starting polling');
                this.startPolling();
            } catch (err) {
                console.error('[HardwareStore] Handshake failed:', err);
                this.isConnected = false;
                this.lastError = String(err);
            } finally {
                this.isConnecting = false;
            }
        },

        startAutoReconnect() {
            if (this.reconnectTimer) return;
            this.reconnectTimer = setInterval(async () => {
                if (!this.isConnected && !this.isConnecting) {
                    await this.connect();
                }
            }, 2000);
        },

        async startPolling() {
            if (!this.isConnected) return;

            // Poll at maximum speed using requestAnimationFrame (~120Hz backend updates)
            const poll = async () => {
                if (!this.isConnected) return;
                try {
                    this.status = await HardwareService.pollStatus();
                    requestAnimationFrame(poll);
                } catch (err) {
                    console.error('Polling error:', err);
                    this.isConnected = false;
                    this.lastError = 'Connection lost';
                }
            };

            requestAnimationFrame(poll);
        },

        async updateFX(newFx: Partial<EffectSettings>) {
            if (!this.effects) return;
            this.effects = { ...this.effects, ...newFx };
            await HardwareService.updateEffectSettings(this.effects);
        },

        async updateHW(newHw: Partial<HardwareSettings>) {
            if (!this.hardware) return;
            this.hardware = { ...this.hardware, ...newHw };
            await HardwareService.updateHardwareSettings(this.hardware);
        },

        async reboot() {
            await HardwareService.reboot();
        },

        async resetCenter() {
            await HardwareService.resetCenter();
        },

        async saveToEeprom() {
            await HardwareService.saveToEeprom();
        },

        async sendFFBTest(type: number, value: number) {
            await HardwareService.sendDirectControl(type, value);
        },

        async enterDfu() {
            await HardwareService.switchToDfu();
        },

        async activateLicense(key: string) {
            await HardwareService.activateLicense(key);
        },

        async updateGPIO(newGpio: Partial<GpioSettings>) {
            if (!this.gpio) return;
            this.gpio = { ...this.gpio, ...newGpio };
            await HardwareService.updateGpioSettings(this.gpio);
        },

        async updateADC(newAdc: Partial<AdcSettings>) {
            if (!this.adc) return;
            this.adc = { ...this.adc, ...newAdc };
            await HardwareService.updateAdcSettings(this.adc);
        },

        async updateKeyboardMapping(mappings: any[]) {
            await HardwareService.setKeyboardMapping(mappings);
        }
    }
});
