import { translate } from '../services/i18n.js';
import { HardwareService } from '../services/HardwareService.js';
import { createSliderRow, attachSliderEvents } from './SettingsWidgets.js';
import { showToast } from '../utils/ui.js';

export const InputsTab = {
    render(container, currentAdcSettings, currentGpioSettings, axisNames, onEditMapping) {
        if (!container || !currentAdcSettings) return;

        const activeIndices = [0, 1, 2, 3, 4, 5].filter(i => {
            if (i < 3) return true;
            return currentGpioSettings && currentGpioSettings.pin_mode && currentGpioSettings.pin_mode[i] === 2;
        });

        container.innerHTML = activeIndices.map(i => {
            const hasHardwareConfig = i < 3;
            const axisLabel = i < 3 ? translate('setting_axis_' + ['x', 'y', 'z'][i]) : (translate('pin_label') + ' ' + i);

            return `
                <div class="card">
                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:1rem;">
                        <div style="display:flex; flex-direction:column;">
                            <span style="font-size:0.7rem; color:var(--text-dim); text-transform:uppercase;">${axisLabel}</span>
                            <span style="font-size:1.1rem; font-weight:700; color:var(--accent);">${axisNames[i]}</span>
                        </div>
                        <button class="btn-outline edit-mapping-btn" data-idx="${i}" style="font-size:0.7rem; display:flex; align-items:center;">
                            <img src="./assets/icons/edit.svg" class="edit-icon" width="14" style="margin-right:4px;">
                            ${translate('btn_edit_mapping')}
                        </button>
                    </div>
                    
                    ${hasHardwareConfig ? `
                    <div class="adc-config-section" id="adc-config-${i}">
                        ${createSliderRow({ key: 'raxis_min', label: 'setting_min', min: 0, max: 32767, step: 1, unit: '' }, { raxis_min: currentAdcSettings.raxis_min[i] }, `adc-${i}`)}
                        ${createSliderRow({ key: 'raxis_max', label: 'setting_max', min: 0, max: 32767, step: 1, unit: '' }, { raxis_max: currentAdcSettings.raxis_max[i] }, `adc-${i}`)}
                    </div>
                    ` : `<div style="font-size:0.7rem; color:var(--text-dim); font-style:italic;">Hardware calibration only for primary axes.</div>`}
                </div>
            `;
        }).join('');

        container.querySelectorAll('.edit-mapping-btn').forEach(btn => {
            btn.onclick = (e) => onEditMapping(parseInt(e.currentTarget.dataset.idx));
        });

        activeIndices.forEach(i => {
            if (i < 3) {
                const valSet = { raxis_min: currentAdcSettings.raxis_min[i], raxis_max: currentAdcSettings.raxis_max[i] };
                attachSliderEvents(container.querySelector(`#adc-config-${i}`), valSet, `adc-${i}`, () => {
                    currentAdcSettings.raxis_min[i] = valSet.raxis_min;
                    currentAdcSettings.raxis_max[i] = valSet.raxis_max;
                    HardwareService.updateAdcSettings(currentAdcSettings);
                });
            }
        });
    }
};
