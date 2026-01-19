import { translate } from '../services/i18n.js';
import { HardwareService } from '../services/HardwareService.js';
import { createSliderRow, createCheckboxRow, attachSliderEvents, attachCheckboxEvents } from './SettingsWidgets.js';
import { showToast } from '../utils/ui.js';

export const HardwareTab = {
    render(container, settings, elements) {
        if (!container || !settings) return;
        container.innerHTML = `
            <div class="setting-group card">
                <h3>${translate('group_motor')}</h3>
                ${[
                { key: 'power_limit', label: 'setting_power_limit', min: 0, max: 100, step: 1, unit: '%' },
                { key: 'braking_limit', label: 'setting_braking_limit', min: 0, max: 255, step: 1, unit: '' },
                { key: 'amplifier_gain', label: 'setting_amplifier_gain', min: 0, max: 255, step: 1, unit: '' },
                { key: 'pole_pairs', label: 'setting_pole_pairs', min: 1, max: 100, step: 1, unit: '' },
                { key: 'calibration_speed', label: 'setting_calibration_speed', min: 0, max: 255, step: 1, unit: '' },
                { key: 'calibration_magnitude', label: 'setting_calibration_magnitude', min: 0, max: 255, step: 1, unit: '' }
            ].map(i => createSliderRow(i, settings, 'hw')).join('')}
            
            ${[
                { key: 'force_enabled', label: 'setting_force_enabled', trueValue: 1, falseValue: 0 },
                { key: 'encoder_direction', label: 'setting_encoder_dir', trueValue: -1, falseValue: 1 },
                { key: 'force_direction', label: 'setting_force_dir', trueValue: -1, falseValue: 1 },
                { key: 'debug_torque', label: 'setting_debug_torque', trueValue: 1, falseValue: 0 }
            ].map(i => createCheckboxRow(i, settings, 'hw')).join('')}
            </div>
            <div class="setting-group card">
                <h3>${translate('group_pid')}</h3>
                ${[
                { key: 'proportional_gain', label: 'setting_p_gain', min: 0, max: 255, step: 1, unit: '' },
                { key: 'integral_gain', label: 'setting_i_gain', min: 0, max: 2000, step: 1, unit: '' }
            ].map(i => createSliderRow(i, settings, 'hw')).join('')}
            </div>
        `;

        attachSliderEvents(container, settings, 'hw', () => HardwareService.updateHardwareSettings(settings));
        attachCheckboxEvents(container, settings, 'hw', () => HardwareService.updateHardwareSettings(settings));
    }
};
