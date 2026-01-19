import { translate } from '../services/i18n.js';
import { HardwareService } from '../services/HardwareService.js';
import { showToast } from '../utils/ui.js';

export const ToolsTab = {
    render(container) {
        if (!container) return;

        container.innerHTML = `
            <div class="card wide">
                <h3>${translate('tools_ffb_test')}</h3>
                <div class="setting-row vertical">
                    <label class="setting-label">${translate('tools_test_profile')}</label>
                    <select id="ffb-test-profile" class="input-field" style="background:#111; border:1px solid #333; border-radius:4px; color:white; padding:8px; width:100%;">
                        <option value="constant">${translate('profile_constant')}</option>
                        <option value="sine">${translate('profile_sine')}</option>
                        <option value="square">${translate('profile_square')}</option>
                    </select>
                </div>

                <div class="setting-row vertical">
                    <label class="setting-label">${translate('tools_force_strength')}</label>
                    <div style="display: flex; gap: 10px; align-items: center;">
                        <input type="range" id="ffb-force-slider" min="-100" max="100" value="0" class="slider" style="flex: 1;">
                        <span id="ffb-force-value" class="setting-value">0%</span>
                    </div>
                </div>

                <div style="margin-top: 1rem; padding: 0.8rem; background: rgba(255, 193, 7, 0.1); border-left: 3px solid #ffc107; border-radius: 4px;">
                    <p style="font-size: 0.75rem; color: var(--text-dim); margin: 0;">
                        ⚠️ ${translate('tools_warning_ffb')}
                    </p>
                </div>
            </div>

            <div class="card">
                <h3>${translate('tools_dfu_mode')}</h3>
                <p style="font-size: 0.8rem; color: var(--text-dim); margin-bottom: 1rem;">
                    ${translate('tools_dfu_description')}
                </p>
                <button id="btn-dfu-mode" class="btn-outline" style="border-color: #f44336; color: #f44336; width: 100%;">
                    ${translate('btn_enter_dfu')}
                </button>
            </div>
        `;

        this.attachEventListeners(container);
    },

    attachEventListeners(container) {
        // Force slider
        const forceSlider = container.querySelector('#ffb-force-slider');
        const forceValue = container.querySelector('#ffb-force-value');
        const profileSelect = container.querySelector('#ffb-test-profile');

        if (forceSlider && forceValue) {
            forceSlider.addEventListener('input', (e) => {
                const val = parseInt(e.target.value);
                forceValue.textContent = `${val}%`;

                // Map to hardware value
                const hwVal = (val / 100) * 32767;
                const profile = profileSelect.value;
                const forceType = profile === 'constant' ? 1 : profile === 'sine' ? 2 : 3;

                HardwareService.sendDirectControl(forceType, Math.round(hwVal)).catch(() => { });
            });

            // Reset on release
            forceSlider.addEventListener('change', (e) => {
                e.target.value = 0;
                forceValue.textContent = '0%';
                HardwareService.sendDirectControl(1, 0).catch(() => { });
            });
        }

        // DFU button
        const dfuBtn = container.querySelector('#btn-dfu-mode');
        if (dfuBtn) {
            dfuBtn.addEventListener('click', async () => {
                if (confirm(translate('confirm_dfu'))) {
                    try {
                        await HardwareService.switchToDFU();
                        showToast(null, translate('toast_dfu_success'), 'success');
                    } catch (e) {
                        showToast(null, translate('toast_dfu_failed') + ': ' + e, 'error');
                    }
                }
            });
        }
    }
};
