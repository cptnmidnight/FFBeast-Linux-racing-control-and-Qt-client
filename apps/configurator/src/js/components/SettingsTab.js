import { translate } from '../services/i18n.js';

export const SettingsTab = {
    render(container) {
        if (!container) return;

        // Load saved or defaults
        const savedSize = localStorage.getItem('ffb_ui_font_size') || '13';
        const savedPadding = localStorage.getItem('ffb_ui_padding') || '1.0';
        const savedFont = localStorage.getItem('ffb_ui_font_family') || 'Outfit';
        const debugEnabled = localStorage.getItem('ffb_ui_debug_enabled') === 'true';
        const savedAccent = localStorage.getItem('ffb_ui_accent') || '#00d4ff';

        // Toast Defaults
        const savedToastPos = localStorage.getItem('ffb_ui_toast_pos') || 'bottom-right';
        const savedToastMargin = localStorage.getItem('ffb_ui_toast_margin') || '20';

        container.innerHTML = `
            <div class="setting-group card">
                <h3>${translate('settings_ui')}</h3>
                
                <div class="setting-row">
                    <span class="setting-label">${translate('settings_font_size')}</span>
                    <input type="range" min="12" max="36" step="1" value="${savedSize}" id="sets-font-size">
                    <span class="setting-value" id="val-font-size">${savedSize}px</span>
                </div>

                <div class="setting-row">
                    <span class="setting-label">${translate('settings_padding')}</span>
                    <input type="range" min="0.5" max="4" step="0.25" value="${savedPadding}" id="sets-padding">
                    <span class="setting-value" id="val-padding">${savedPadding}rem</span>
                </div>
                
                 <div class="setting-row">
                    <span class="setting-label">${translate('settings_accent_color') || 'Accent Color'}</span>
                    <input type="color" id="sets-accent" value="${savedAccent}" style="background: none; border-radius: 4px; border: 1px solid var(--border); width: 50px; height: 30px; cursor: pointer; padding: 2px;">
                </div>

                 <div class="setting-row vertical">
                    <span class="setting-label">${translate('settings_font_family')}</span>
                    <select id="sets-font-family" class="modal-select">
                        <optgroup label="Modern">
                            <option value="Outfit" ${savedFont === 'Outfit' ? 'selected' : ''}>Outfit (Default)</option>
                            <option value="Inter" ${savedFont === 'Inter' ? 'selected' : ''}>Inter</option>
                            <option value="Roboto" ${savedFont === 'Roboto' ? 'selected' : ''}>Roboto</option>
                        </optgroup>
                        <optgroup label="Windows">
                            <option value="Segoe UI" ${savedFont === 'Segoe UI' ? 'selected' : ''}>Segoe UI</option>
                            <option value="Arial" ${savedFont === 'Arial' ? 'selected' : ''}>Arial</option>
                            <option value="Verdana" ${savedFont === 'Verdana' ? 'selected' : ''}>Verdana</option>
                        </optgroup>
                         <optgroup label="Monospace">
                            <option value="Consolas" ${savedFont === 'Consolas' ? 'selected' : ''}>Consolas</option>
                        </optgroup>
                    </select>
                </div>

                <div class="setting-row">
                    <span class="setting-label">${translate('settings_debug_panel')}</span>
                    <label class="switch">
                        <input type="checkbox" id="sets-debug-log" ${debugEnabled ? 'checked' : ''}>
                        <span class="slider round"></span>
                    </label>
                </div>
            </div>

            <div class="setting-group card">
                <h3>${translate('group_toasts') || 'Notifications'}</h3>
                
                <div class="setting-row vertical">
                    <span class="setting-label">${translate('setting_toast_position') || 'Position'}</span>
                    <select id="sets-toast-pos" class="modal-select">
                        <option value="bottom-right" ${savedToastPos === 'bottom-right' ? 'selected' : ''}>${translate('toast_pos_bottom_right') || 'Bottom Right'}</option>
                        <option value="bottom-left" ${savedToastPos === 'bottom-left' ? 'selected' : ''}>${translate('toast_pos_bottom_left') || 'Bottom Left'}</option>
                        <option value="top-right" ${savedToastPos === 'top-right' ? 'selected' : ''}>${translate('toast_pos_top_right') || 'Top Right'}</option>
                        <option value="top-left" ${savedToastPos === 'top-left' ? 'selected' : ''}>${translate('toast_pos_top_left') || 'Top Left'}</option>
                        <option value="bottom-center" ${savedToastPos === 'bottom-center' ? 'selected' : ''}>${translate('toast_pos_bottom_center') || 'Bottom Center'}</option>
                        <option value="top-center" ${savedToastPos === 'top-center' ? 'selected' : ''}>${translate('toast_pos_top_center') || 'Top Center'}</option>
                    </select>
                </div>

                <div class="setting-row">
                    <span class="setting-label">${translate('setting_toast_margin') || 'Margin'}</span>
                    <input type="range" min="0" max="100" step="5" value="${savedToastMargin}" id="sets-toast-margin">
                    <span class="setting-value" id="val-toast-margin">${savedToastMargin}px</span>
                </div>
            </div>
            
            <div class="setting-group card">
                <h3>${translate('app_info')}</h3>
                <div class="setting-row">
                    <span class="setting-label">${translate('settings_version') || 'Version'}</span>
                    <span class="setting-value">v1.0.0-beta</span>
                </div>
                 <div class="setting-row">
                    <span class="setting-label">${translate('settings_build_context') || 'Build Context'}</span>
                    <span class="setting-value">Production</span>
                </div>
            </div>
        `;

        const updateUI = () => {
            const size = document.getElementById('sets-font-size').value;
            const pad = document.getElementById('sets-padding').value;
            const font = document.getElementById('sets-font-family').value;
            const debug = document.getElementById('sets-debug-log').checked;
            const accent = document.getElementById('sets-accent').value;

            const toastPos = document.getElementById('sets-toast-pos').value;
            const toastMargin = document.getElementById('sets-toast-margin').value;

            document.documentElement.style.setProperty('--font-size-base', `${size}px`);
            document.body.style.fontSize = `${size}px`;
            document.documentElement.style.setProperty('--content-padding', `${pad}rem`);

            const fontVal = font.includes(' ') ? `'${font}', sans-serif` : `${font}, sans-serif`;
            document.documentElement.style.setProperty('--font-sans', fontVal);
            document.documentElement.style.setProperty('--accent', accent);

            SettingsTab.applyDynamicStyles(pad, debugEnabled, toastPos, toastMargin); // Use state for debug to avoid flicker? No, use current value
            SettingsTab.applyDynamicStyles(pad, debug, toastPos, toastMargin);

            document.getElementById('val-font-size').textContent = `${size}px`;
            document.getElementById('val-padding').textContent = `${pad}rem`;
            document.getElementById('val-toast-margin').textContent = `${toastMargin}px`;

            localStorage.setItem('ffb_ui_font_size', size);
            localStorage.setItem('ffb_ui_padding', pad);
            localStorage.setItem('ffb_ui_font_family', font);
            localStorage.setItem('ffb_ui_debug_enabled', debug);
            localStorage.setItem('ffb_ui_accent', accent);
            localStorage.setItem('ffb_ui_toast_pos', toastPos);
            localStorage.setItem('ffb_ui_toast_margin', toastMargin);
        };

        container.querySelectorAll('input, select').forEach(el => {
            el.addEventListener('input', updateUI);
            el.addEventListener('change', updateUI);
        });

        // Initial sync
        const pad = document.getElementById('sets-padding').value;
        const debug = document.getElementById('sets-debug-log').checked;
        SettingsTab.applyDynamicStyles(pad, debug, savedToastPos, savedToastMargin);
    },

    applySavedSettings() {
        const savedSize = localStorage.getItem('ffb_ui_font_size') || '13';
        const savedPadding = localStorage.getItem('ffb_ui_padding') || '1.0';
        const savedFont = localStorage.getItem('ffb_ui_font_family') || 'Outfit';
        const debugEnabled = localStorage.getItem('ffb_ui_debug_enabled') === 'true';
        const savedAccent = localStorage.getItem('ffb_ui_accent') || '#00d4ff';

        const savedToastPos = localStorage.getItem('ffb_ui_toast_pos') || 'bottom-right';
        const savedToastMargin = localStorage.getItem('ffb_ui_toast_margin') || '20';

        document.body.style.fontSize = `${savedSize}px`;
        const fontVal = savedFont.includes(' ') ? `'${savedFont}', sans-serif` : `${savedFont}, sans-serif`;
        document.documentElement.style.setProperty('--font-sans', fontVal);
        document.documentElement.style.setProperty('--accent', savedAccent);
        document.documentElement.style.setProperty('--content-padding', `${savedPadding}rem`);

        this.applyDynamicStyles(savedPadding, debugEnabled, savedToastPos, savedToastMargin);
    },

    applyDynamicStyles(padding, debugEnabled, toastPos = 'bottom-right', toastMargin = '20') {
        let style = document.getElementById('dynamic-settings-style');
        if (!style) {
            style = document.createElement('style');
            style.id = 'dynamic-settings-style';
            document.head.appendChild(style);
        }

        // CSS Style Injection
        style.textContent = `
            .content-header, .settings-grid, .card, .dashboard-layout { padding: ${padding}rem !important; }
            #monitor-log-panel { display: ${debugEnabled ? 'block' : 'none'} !important; }
        `;

        // CSS Variables for Toast (Cleanest way)
        let top = 'auto', bottom = 'auto', left = 'auto', right = 'auto';
        const m = `${toastMargin}px`;
        let transform = 'none';

        if (toastPos.includes('top')) top = m;
        if (toastPos.includes('bottom')) bottom = m;
        if (toastPos.includes('left')) left = m;
        if (toastPos.includes('right')) right = m;
        if (toastPos.includes('center')) {
            left = '50%';
            transform = 'translateX(-50%)';
        }

        const r = document.documentElement.style;
        r.setProperty('--toast-top', top);
        r.setProperty('--toast-bottom', bottom);
        r.setProperty('--toast-left', left);
        r.setProperty('--toast-right', right);
        r.setProperty('--toast-transform', transform);
    }
};
