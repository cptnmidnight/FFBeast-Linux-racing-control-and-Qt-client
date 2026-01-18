import { translate } from '../services/i18n.js';
import { HardwareService } from '../services/HardwareService.js';
import { createSliderRow, attachSliderEvents } from './SettingsWidgets.js';

export const EffectsTab = {
    render(container, settings) {
        if (!container || !settings) return;
        container.innerHTML = `
            <div class="setting-group card">
                <h3>${translate('group_general')}</h3>
                ${[
                { key: 'motion_range', label: 'setting_motion_range', min: 90, max: 1080, step: 1, unit: '°' },
                { key: 'total_effect_strength', label: 'setting_total_strength', min: 0, max: 100, step: 1, unit: '%' },
                { key: 'integrated_spring_strength', label: 'setting_integrated_spring', min: 0, max: 255, step: 1, unit: '' }
            ].map(i => createSliderRow(i, settings, 'fx')).join('')}
            </div>
            <div class="setting-group card">
                <h3>${translate('group_dampening')}</h3>
                ${[
                { key: 'static_dampening_strength', label: 'setting_static_dampening', min: 0, max: 1000, step: 1, unit: '' },
                { key: 'dynamic_dampening_strength', label: 'setting_dynamic_dampening', min: 0, max: 1000, step: 1, unit: '' }
            ].map(i => createSliderRow(i, settings, 'fx')).join('')}
            </div>
            <div class="setting-group card">
                <h3>${translate('group_direct_x')}</h3>
                ${[
                { key: 'direct_x_constant_strength', label: 'setting_dx_constant', min: 0, max: 100, step: 1, unit: '%' },
                { key: 'direct_x_periodic_strength', label: 'setting_dx_periodic', min: 0, max: 100, step: 1, unit: '%' },
                { key: 'direct_x_spring_strength', label: 'setting_dx_spring', min: 0, max: 100, step: 1, unit: '%' }
            ].map(i => createSliderRow(i, settings, 'fx')).join('')}
            </div>
        `;
        attachSliderEvents(container, settings, 'fx', () => HardwareService.updateEffectSettings(settings));
    }
};
