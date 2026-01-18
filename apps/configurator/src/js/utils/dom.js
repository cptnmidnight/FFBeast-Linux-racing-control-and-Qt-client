export const $ = (selector) => document.querySelector(selector);
export const $$ = (selector) => document.querySelectorAll(selector);

export function refreshElementCache() {
    return {
        navItems: $$('.nav-item'),
        tabContents: $$('.tab-content'),
        tabTitle: $('#tab-title'),
        statusText: $('#status-text'),
        connectionStatus: $('#connection-status'),
        // Dashboard
        wheelRotateGroup: $('#wheel-rotate'),
        wheelPosValue: $('#wheel-pos-value'),
        torqueBarV: $('#torque-bar-v'),
        torqueValueNode: $('#torque-value'),
        monitorButtons: $('#monitor-buttons-grid'),
        monitorAnalog: $('#monitor-analog-container'),
        // Misc
        toastContainer: $('#toast-container'),
        firmwareVer: $('#firmware-ver'),
        btnSave: $('#btn-save'),
        btnResetCenter: $('#btn-reset-center'),
        sysStatus: $('#sys-status'),
        tooltip: $('#global-tooltip')
    };
}
