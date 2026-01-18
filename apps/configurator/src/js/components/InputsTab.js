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
                        
                        <div class="setting-row" style="display:flex; justify-content:space-between; align-items:center; margin-top:10px;">
                             <span class="setting-label">${translate('setting_axis_invert')} <img src="./assets/icons/help.svg" class="help-icon" data-help="help_axis_invert"></span>
                             <label class="switch" style="position:relative; display:inline-block; width:40px; height:20px;">
                                <input type="checkbox" id="chk-adc-${i}-invert" 
                                       class="adc-invert-chk"
                                       data-idx="${i}"
                                       ${currentAdcSettings.raxis_invert[i] ? 'checked' : ''}
                                       style="opacity:0; width:0; height:0;">
                                <span class="slider round" style="position:absolute; cursor:pointer; top:0; left:0; right:0; bottom:0; background-color:#333; transition:.4s; border-radius:34px;"></span>
                            </label>
                            <style>
                                .switch input:checked + .slider { background-color: var(--accent); }
                                .switch input:focus + .slider { box-shadow: 0 0 1px var(--accent); }
                                .switch input:checked + .slider:before { transform: translateX(20px); }
                                .slider:before { position: absolute; content: ""; height: 16px; width: 16px; left: 2px; bottom: 2px; background-color: white; transition: .4s; border-radius: 50%; }
                            </style>
                        </div>
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

                const invertChk = container.querySelector(`#chk-adc-${i}-invert`);
                if (invertChk) {
                    invertChk.onchange = (e) => {
                        currentAdcSettings.raxis_invert[i] = e.target.checked ? 1 : 0;
                        HardwareService.updateAdcSettings(currentAdcSettings);
                    };
                }
            }
        });
    }
};
