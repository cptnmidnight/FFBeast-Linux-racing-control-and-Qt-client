import { translate } from '../services/i18n.js';
import { HardwareService } from '../services/HardwareService.js';
import { showToast } from '../utils/ui.js';

export class ModalManager {
    constructor(elements, axisNames, axisKeyMapping, currentAdcSettings, keyOptions) {
        this.elements = elements;
        this.axisNames = axisNames;
        this.axisKeyMapping = axisKeyMapping;
        this.currentAdcSettings = currentAdcSettings;
        this.keyOptions = keyOptions;
        this.currentMappingIdx = -1;
    }

    openMappingModal(idx, onSave) {
        this.currentMappingIdx = idx;
        const modal = document.getElementById('modal-axis-mapping');
        const inputName = document.getElementById('modal-axis-name');
        const btnLow = document.getElementById('modal-btn-low');
        const btnHigh = document.getElementById('modal-btn-high');
        const keyLow = document.getElementById('modal-key-low');
        const keyHigh = document.getElementById('modal-key-high');

        if (!modal || !inputName) return;

        inputName.value = this.axisNames[idx] || '';

        const btnOptions = `<option value="0">${translate('btn_none')}</option>` +
            Array.from({ length: 32 }, (_, i) => `<option value="${i + 1}">${translate('btn_label')} ${i + 1}</option>`).join('');
        btnLow.innerHTML = btnOptions;
        btnHigh.innerHTML = btnOptions;

        btnLow.value = this.currentAdcSettings.raxis_to_button_low[idx] || 0;
        btnHigh.value = this.currentAdcSettings.raxis_to_button_high[idx] || 0;

        const keyHtml = `<option value="">${translate('btn_none')}</option>` +
            this.keyOptions.map(k => `<option value="${k}">${k}</option>`).join('');
        keyLow.innerHTML = keyHtml;
        keyHigh.innerHTML = keyHtml;

        const keyMap = this.axisKeyMapping[idx] || {};
        keyLow.value = keyMap.low || '';
        keyHigh.value = keyMap.high || '';

        modal.classList.add('active');

        document.getElementById('modal-save').onclick = () => {
            this.axisNames[this.currentMappingIdx] = inputName.value;
            localStorage.setItem('ffbeast_axis_names', JSON.stringify(this.axisNames));

            if (this.currentMappingIdx < 3) {
                this.currentAdcSettings.raxis_to_button_low[this.currentMappingIdx] = parseInt(btnLow.value);
                this.currentAdcSettings.raxis_to_button_high[this.currentMappingIdx] = parseInt(btnHigh.value);
                HardwareService.updateAdcSettings(this.currentAdcSettings);
            }

            this.axisKeyMapping[this.currentMappingIdx] = {
                low: keyLow.value,
                high: keyHigh.value
            };
            localStorage.setItem('ffbeast_axis_keys', JSON.stringify(this.axisKeyMapping));

            showToast(this.elements, translate('btn_save'), 'success');
            modal.classList.remove('active');
            if (onSave) onSave();
        };

        document.getElementById('modal-close').onclick = () => modal.classList.remove('active');
    }
}
