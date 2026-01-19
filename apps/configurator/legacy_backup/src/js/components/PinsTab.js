import { translate } from '../services/i18n.js';
import { HardwareService } from '../services/HardwareService.js';

export const PinsTab = {
    render(container, currentGpioSettings) {
        if (!container || !currentGpioSettings) return;
        const pinModes = ["none", "gpio", "analog", "spi_cs", "spi_sck", "spi_miso", "enable_effects", "center_reset", "braking_pwm", "effect_led", "reboot"];
        container.innerHTML = `
            <div class="card wide">
                <h3>${translate('group_pins')}</h3>
                <div style="display:flex; flex-direction:column; gap: 4px;">
                    ${currentGpioSettings.pin_mode.map((mode, i) => `
                        <div class="setting-row vertical">
                            <span class="setting-label" style="font-size:0.7rem; color:var(--text-dim);">${translate('pin_label')} ${i}</span>
                            <select class="pin-sel" data-idx="${i}" style="background:#111; border:1px solid #333; border-radius:4px; color:white; width:100%; font-size:0.7rem; padding:2px;">
                                ${pinModes.map((m, idx) => `<option value="${idx}" ${mode === idx ? 'selected' : ''}>${translate('pin_mode_' + m)}</option>`).join('')}
                            </select>
                        </div>
                    `).join('')}
                </div>
            </div>
        `;
        container.querySelectorAll('.pin-sel').forEach(s => s.onchange = (e) => {
            currentGpioSettings.pin_mode[parseInt(e.target.dataset.idx)] = parseInt(e.target.value);
            HardwareService.updateGpioSettings(currentGpioSettings);
        });
    }
};
