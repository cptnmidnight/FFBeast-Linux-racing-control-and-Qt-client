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
            debugMode: false
        }
    }),

    actions: {
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
