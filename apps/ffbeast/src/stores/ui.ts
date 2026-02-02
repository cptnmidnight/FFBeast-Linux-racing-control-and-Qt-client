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
            toastPosition: (localStorage.getItem('ffbeast_toast_position') || 'bottom-right') as 'top-right' | 'bottom-right' | 'top-left' | 'bottom-left',
            toastMargin: Number(localStorage.getItem('ffbeast_toast_margin') || '20'),
            fontFamily: localStorage.getItem('ffbeast_font') || 'Outfit',
            fontSize: Number(localStorage.getItem('ffbeast_font_size') || '16'),
            accentColor: localStorage.getItem('ffbeast_accent') || '#00d4ff',
            customAccentColor: localStorage.getItem('ffbeast_custom_accent') || '#00d4ff',
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
        },

        setLanguage(lang: string) {
            localStorage.setItem('ffbeast_language', lang);
        },

        setFontFamily(font: string) {
            this.settings.fontFamily = font;
            document.documentElement.style.setProperty('--font-main', font);
            localStorage.setItem('ffbeast_font', font);
        },

        setFontSize(size: number) {
            this.settings.fontSize = size;
            document.documentElement.style.fontSize = `${size}px`;
            localStorage.setItem('ffbeast_font_size', String(size));
        },

        setAccentColor(color: string) {
            this.settings.accentColor = color;
            document.documentElement.style.setProperty('--accent', color);
            document.documentElement.style.setProperty('--accent-glow', color + '66');

            const textColor = this.getContrastColor(color);
            document.documentElement.style.setProperty('--text-on-accent', textColor);

            localStorage.setItem('ffbeast_accent', color);
        },

        getContrastColor(hexColor: string) {
            const hex = hexColor.replace('#', '');
            if (hex.length !== 6) return 'black'; // Fallback

            const r = parseInt(hex.substr(0, 2), 16);
            const g = parseInt(hex.substr(2, 2), 16);
            const b = parseInt(hex.substr(4, 2), 16);

            const yiq = ((r * 299) + (g * 587) + (b * 114)) / 1000;
            return (yiq >= 128) ? '#1a1a1a' : '#ffffff';
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
