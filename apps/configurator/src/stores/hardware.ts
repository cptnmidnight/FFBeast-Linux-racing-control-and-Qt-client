import { defineStore } from 'pinia';
import { HardwareService } from '../services/hardware_service';
import type { HardwareStatus } from '../models/HardwareStatus';
import type { EffectSettings } from '../models/EffectSettings';
import type { HardwareSettings } from '../models/HardwareSettings';
import type { GpioSettings } from '../models/GpioSettings';

export const useHardwareStore = defineStore('hardware', {
    state: () => ({
        isConnected: false,
        isConnecting: false,
        status: null as HardwareStatus | null,
        effects: null as EffectSettings | null,
        hardware: null as HardwareSettings | null,
        gpio: null as GpioSettings | null,
        lastError: null as string | null,
        reconnectTimer: null as ReturnType<typeof setInterval> | null,
    }),

    actions: {
        async init() {
            await this.connect();
            this.startAutoReconnect();
        },

        async connect() {
            if (this.isConnecting || this.isConnected) return;
            console.log('Attempting to connect to hardware...');
            this.isConnecting = true;
            try {
                const data = await HardwareService.getHandshake();
                console.log('Handshake successful:', data);
                this.status = data.status;
                this.effects = data.fx;
                this.hardware = data.hw;
                this.gpio = data.gpio;
                this.isConnected = true;
                this.lastError = null;
                this.startPolling();
            } catch (err) {
                console.error('Handshake failed:', err);
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

            // Limit polling frequency to ~60Hz
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
        }
    }
});
