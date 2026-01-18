import { translate } from '../services/i18n.js';
import { HardwareService } from '../services/HardwareService.js';

export const ButtonsTab = {
    render(container, currentGpioSettings) {
        if (!container || !currentGpioSettings) return;
        const btnModes = ["none", "normal", "inverted", "pulse"];
        container.innerHTML = `
            <div class="card wide">
                <h3>${translate('group_buttons')}</h3>
                <div style="display:grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 8px;">
                    ${currentGpioSettings.button_mode.slice(0, 32).map((mode, i) => `
                        <div class="setting-row vertical">
                            <span class="setting-label" style="font-size:0.7rem; color:var(--text-dim);">${translate('btn_label')} ${i + 1}</span>
                            <select class="btn-sel" data-idx="${i}" style="background:#111; border:1px solid #333; border-radius:4px; color:white; width:100%; font-size:0.7rem; padding:2px;">
                                ${btnModes.map((m, idx) => `<option value="${idx}" ${mode === idx ? 'selected' : ''}>${translate('btn_mode_' + m)}</option>`).join('')}
                            </select>
                        </div>
                    `).join('')}
                </div>
            </div>
        `;
        container.querySelectorAll('.btn-sel').forEach(s => s.onchange = (e) => {
            currentGpioSettings.button_mode[parseInt(e.target.dataset.idx)] = parseInt(e.target.value);
            HardwareService.updateGpioSettings(currentGpioSettings);
        });
    }
};
