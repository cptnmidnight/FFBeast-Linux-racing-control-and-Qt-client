import { translate } from '../services/i18n.js';
import { HardwareService } from '../services/HardwareService.js';
import { showToast } from '../utils/ui.js';

export const LicenseTab = {
    render(container, elements) {
        if (!container) return;

        container.innerHTML = `
            <div class="card wide">
                <h3 data-i18n="license_info">${translate('license_info')}</h3>
                <div class="setting-row">
                    <span class="setting-label" data-i18n="device_id">${translate('device_id')}</span>
                    <div style="display:flex; align-items:center; gap:10px;">
                        <code id="lic-tab-id"
                            style="background:#000; padding:4px 8px; border-radius:4px; color:var(--accent); font-family:monospace;">-</code>
                        <button class="btn-outline narrow" id="btn-copy-id" data-help="tooltip_copy_id">
                            <img src="./assets/icons/copy.svg" width="14" class="icon-white">
                        </button>
                    </div>
                </div>
                <div class="setting-row">
                    <span class="setting-label" data-i18n="license_status">${translate('license_status')}</span>
                    <span id="lic-tab-status" class="status-badge">-</span>
                </div>
            </div>
            <div class="card wide">
                <h3 data-i18n="license_activation">${translate('license_activation')}</h3>
                <div class="setting-row vertical">
                    <label class="setting-label" data-i18n="serial_key">${translate('serial_key')}</label>
                    <textarea id="input-serial-key" rows="3"
                        style="width:100%; background:#111; color:white; border:1px solid #333; border-radius:4px; padding:8px; font-family:monospace;"
                        placeholder="${translate('serial_placeholder') || 'Paste Serial Key...'}"></textarea>
                </div>
                <div style="display:flex; justify-content:flex-end; gap:10px; margin-top:1rem;">
                    <button id="btn-activate" class="btn-primary" data-i18n="btn_activate">${translate('btn_activate')}</button>
                </div>
            </div>
        `;

        this.attachEvents(container, elements);

        // Trigger a fake status update to fill ID/Status if already connected
        // In a real scenario, Monitor.js will fill this on next packet or we can pull from HardwareService
    },

    attachEvents(container, elements) {
        const btnCopy = container.querySelector('#btn-copy-id');
        if (btnCopy) {
            btnCopy.onclick = () => {
                const txt = container.querySelector('#lic-tab-id')?.textContent || "";
                if (txt && txt !== '-') {
                    navigator.clipboard.writeText(txt);
                    showToast(elements, translate('toast_id_copied'), 'info');
                }
            };
        }

        const btnActivate = container.querySelector('#btn-activate');
        const inputKey = container.querySelector('#input-serial-key');
        if (btnActivate && inputKey) {
            btnActivate.onclick = async () => {
                const key = inputKey.value;
                if (!key || key.trim().length < 10) {
                    showToast(elements, translate('toast_invalid_license'), 'error');
                    return;
                }
                try {
                    await HardwareService.activateLicense(key);
                    showToast(elements, translate('toast_license_activated'), 'success');
                    // Refresh data after a short delay to see new status
                    setTimeout(() => {
                        window.location.reload(); // Simplest way to ensure everything re-syncs
                    }, 2000);
                } catch (e) {
                    showToast(elements, translate('toast_activation_failed') + ": " + e, 'error');
                }
            };
        }
    }
};
