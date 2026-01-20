import { defineStore } from 'pinia';

interface Toast {
    id: number;
    message: string;
    type: 'info' | 'success' | 'warn' | 'error';
    duration?: number;
}

export const useUIStore = defineStore('ui', {
    state: () => ({
        toasts: [] as Toast[],
        tooltip: {
            show: false,
            text: '',
            x: 0,
            y: 0
        },
        settings: {
            toastPosition: 'bottom-right' as 'top-right' | 'bottom-right' | 'top-left' | 'bottom-left',
            toastMargin: 20,
            fontFamily: 'Outfit',
            fontSize: 16,
            minLogLevel: Number(localStorage.getItem('ffbeast_min_log_level') || '3'),
            debugMode: localStorage.getItem('ffbeast_debug_mode') === 'true'
        }
    }),

    actions: {
        async setMinLogLevel(level: number) {
            this.settings.minLogLevel = level;
            localStorage.setItem('ffbeast_min_log_level', String(level));
            try {
                const { HardwareService } = await import('../services/hardware_service');
                await HardwareService.setMinLogLevel(level);
            } catch (err) {
                console.error('Failed to sync log level:', err);
            }
        },

        async toggleDebugMode(enabled: boolean) {
            this.settings.debugMode = enabled;
            localStorage.setItem('ffbeast_debug_mode', String(enabled));
            // When legacy debug mode is toggled, it might influence the min log level
            // but for now we just keep the boolean for UI compatibility if needed
        },
        showToast(message: string, type: Toast['type'] = 'info', duration = 3000) {
            const id = Date.now();
            this.toasts.push({ id, message, type, duration });
            setTimeout(() => {
                this.removeToast(id);
            }, duration);
        },

        removeToast(id: number) {
            this.toasts = this.toasts.filter(t => t.id !== id);
        },

        showTooltip(text: string, x: number, y: number) {
            this.tooltip = { show: true, text, x, y };
        },

        hideTooltip() {
            this.tooltip.show = false;
        }
    }
});
