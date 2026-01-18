import { translate } from '../services/i18n.js';
import { HardwareService } from '../services/HardwareService.js';

export const ProtocolTab = {
    render(container, currentGpioSettings) {
        if (!container || !currentGpioSettings) return;
        const currentMode = currentGpioSettings.extension_mode || 0;
        const titleKeys = ['mode_none', 'mode_buttons', 'mode_spi_tm', 'mode_spi_fanatec'];
        const detailKeys = ['none', 'buttons', 'tm', 'fanatec'];

        const titleKey = titleKeys[currentMode] || 'mode_none';
        const detailKey = detailKeys[currentMode] || 'none';

        container.innerHTML = `
            <div class="card wide">
                <h3>${translate('group_extension')}</h3>
                <div class="setting-row">
                    <span class="setting-label">${translate('setting_ext_mode')} <img src="./assets/icons/help.svg" class="help-icon" data-help="help_ext_mode"></span>
                    <select id="sel-ext-mode" style="background:#111; border:1px solid #333; border-radius:4px; color:white; padding:4px; font-size:0.8rem; margin-top:0.5rem; width:100%;">
                        <option value="0" ${currentMode === 0 ? 'selected' : ''}>${translate('mode_none')}</option>
                        <option value="1" ${currentMode === 1 ? 'selected' : ''}>${translate('mode_buttons')}</option>
                        <option value="2" ${currentMode === 2 ? 'selected' : ''}>${translate('mode_spi_tm')}</option>
                        <option value="3" ${currentMode === 3 ? 'selected' : ''}>${translate('mode_spi_fanatec')}</option>
                    </select>
                </div>
            </div>
            
            <div class="card wide protocol-info-box">
                <h3 style="color: var(--accent); margin-bottom: 1rem;">📘 ${translate(titleKey)}</h3>
                <div class="protocol-info-section">
                    <h4>Description</h4>
                    <p>${translate('protocol_desc_' + detailKey)}</p>
                </div>
                <div class="protocol-info-section">
                    <h4>Compatibility</h4>
                    <p>${translate('protocol_compat_' + detailKey)}</p>
                </div>
                <div class="protocol-info-section">
                    <h4>Configuration</h4>
                    <p>${translate('protocol_config_' + detailKey)}</p>
                </div>
                <div class="protocol-info-section">
                    <h4>💡 Tips & Troubleshooting</h4>
                    <p>${translate('protocol_tips_' + detailKey)}</p>
                </div>
            </div>
        `;

        container.querySelector('#sel-ext-mode').onchange = (e) => {
            currentGpioSettings.extension_mode = parseInt(e.target.value);
            HardwareService.updateGpioSettings(currentGpioSettings);
            this.render(container, currentGpioSettings);
        };
    }
};
