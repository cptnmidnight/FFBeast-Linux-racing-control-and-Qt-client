export const Logger = {
    logs: [],
    listeners: [],

    init() {
        if (this.initialized) return;
        this.initialized = true;

        const originalLog = console.log;
        const originalWarn = console.warn;
        const originalError = console.error;
        const originalInfo = console.info;

        const addLog = (type, args) => {
            try {
                const msg = args.map(a => {
                    if (a instanceof Error) return a.toString();
                    if (typeof a === 'object') {
                        try { return JSON.stringify(a); } catch (e) { return '[Object]'; }
                    }
                    return String(a);
                }).join(' ');

                const entry = { time: new Date().toLocaleTimeString(), type, msg };
                this.logs.push(entry);
                if (this.logs.length > 500) this.logs.shift();
                this.notify(entry);
            } catch (e) {
                originalError.call(console, 'Logger error:', e);
            }
        };

        console.log = (...args) => { addLog('log', args); originalLog.apply(console, args); };
        console.warn = (...args) => { addLog('warn', args); originalWarn.apply(console, args); };
        console.error = (...args) => { addLog('error', args); originalError.apply(console, args); };
        console.info = (...args) => { addLog('info', args); originalInfo.apply(console, args); };

        console.log('[Logger] System initialized');
    },

    subscribe(callback) {
        this.listeners.push(callback);
        // Replay history so new subscribers see past logs
        this.logs.forEach(entry => callback(entry));
    },

    notify(entry) {
        this.listeners.forEach(cb => cb(entry));
    },

    getLogs() {
        return this.logs;
    },

    renderEntry(entry) {
        const div = document.createElement('div');
        div.className = `log-entry log-${entry.type}`;
        div.innerHTML = `<span class="log-time">[${entry.time}]</span> <span class="log-msg">${entry.msg}</span>`;
        return div;
    }
};
