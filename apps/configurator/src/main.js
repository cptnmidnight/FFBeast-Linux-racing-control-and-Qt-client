import { translate, detectLanguage, loadTranslations } from './i18n.js';

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

let currentTab = 'monitor';
let isConnected = false;
let language = detectLanguage();

// Current hardware state
let currentEffectSettings = null;
let currentHardwareSettings = null;
let currentGpioSettings = null;
let currentAdcSettings = null;

// Persistence for UI-only settings (like axis names)
let savedNames = JSON.parse(localStorage.getItem('ffbeast_axis_names') || '[]');
const defaultNames = ["Throttle", "Brake", "Clutch", "A4", "A5", "A6"];
let axisNames = defaultNames.map((name, i) => savedNames[i] || name);
let axisKeyMapping = JSON.parse(localStorage.getItem('ffbeast_axis_keys') || '[]');
if (axisKeyMapping.length < 6) {
    while (axisKeyMapping.length < 6) axisKeyMapping.push({});
}

const KEY_OPTIONS = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
    "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "Up", "Down", "Left", "Right", "Space", "Enter", "Tab", "Shift", "Ctrl", "Alt", "Esc"
];

// DOM Elements cache
let elements = {};

function refreshElementCache() {
    elements = {
        navItems: document.querySelectorAll('.nav-item'),
        tabContents: document.querySelectorAll('.tab-content'),
        tabTitle: document.getElementById('tab-title'),
        statusText: document.getElementById('status-text'),
        connectionStatus: document.getElementById('connection-status'),
        // Dashboard
        wheelRotateGroup: document.getElementById('wheel-rotate'),
        wheelPosValue: document.getElementById('wheel-pos-value'),
        torqueBarV: document.getElementById('torque-bar-v'),
        torqueValueNode: document.getElementById('torque-value'),
        monitorButtons: document.getElementById('monitor-buttons-grid'),
        monitorAnalog: document.getElementById('monitor-analog-container'),
        // Misc
        toastContainer: document.getElementById('toast-container'),
        firmwareVer: document.getElementById('firmware-ver'),
        btnSave: document.getElementById('btn-save'),
        btnResetCenter: document.getElementById('btn-reset-center'),
        sysStatus: document.getElementById('sys-status'),
        tooltip: document.getElementById('global-tooltip')
    };
}

async function init() {
    try {
        console.info('[Configurator] Starting initialization...');

        if (!window.__TAURI__) {
            console.error('[Configurator] Tauri API not found! Are you running in a browser instead of the Tauri app?');
        }

        await loadTranslations(language);
        console.log('[Configurator] Translations loaded');

        refreshElementCache();
        console.log('[Configurator] Element cache refreshed');

        applyTranslations();
        console.log('[Configurator] Translations applied');

        setupEventListeners();
        console.log('[Configurator] Event listeners setup');
        setupLicenseUI();
        // Initial fetch
        await loadAllData();

        // initMonitorVisuals();
        console.log('[Configurator] Monitor visuals initialization skipped (handled by app.js)');

        setupTooltips();
        console.log('[Configurator] Tooltips setup');

        console.log('[Configurator] Starting background checks...');
        attemptAutoConnect();

        try {
            // DEPRECATED: Listener moved to app.js (HardwareService.onStatusUpdate)
            /*
            await listen('wheel-status', (event) => {
                if (!event.payload) return;

                if (!window._firstPacketLogged) {
                    console.log('[Telemetry] First packet received:', event.payload);
                    window._firstPacketLogged = true;
                }

                if (!isConnected) {
                    console.info('[Configurator] First status received from hardware!');
                    isConnected = true;
                    updateConnectionUI();
                    loadAllData();
                }
                updateMonitorVisuals(event.payload);
            });
            console.log('[Configurator] Hardware event listener active');
            */
            console.log('[Configurator] Legacy event listener disabled (using app.js)');
        } catch (e) {
            console.warn('[Configurator] Failed to setup event listener (Telemetry will be disabled):', e);
        }

    } catch (e) {
        console.error('[Configurator] CRITICAL INIT ERROR:', e);
    }

    // Interval check remains outside the try/catch of init but inside the init function to ensure it runs
    setInterval(async () => {
        try {
            const connected = await invoke('check_hardware');
            if (connected !== isConnected) {
                console.info(`[Configurator] Connection toggle detected: ${isConnected} -> ${connected}`);
                isConnected = connected;
                updateConnectionUI();
                if (isConnected) {
                    loadAllData();
                }
            }
        } catch (e) {
            console.error('[Configurator] check_hardware failed:', e);
        }
        if (!isConnected) {
            attemptAutoConnect();
        }
    }, 2000);
}

function initMonitorVisuals() {
    if (!elements.monitorButtons) refreshElementCache();

    if (elements.monitorButtons) {
        elements.monitorButtons.innerHTML = Array.from({ length: 32 }, (_, i) => `
            <div class="btn-indicator" id="mon-btn-${i}">${i + 1}</div>
        `).join('');
    }
    if (elements.monitorAnalog) {
        elements.monitorAnalog.innerHTML = [0, 1, 2, 3, 4, 5].map(i => `
            <div class="analog-v-col" id="mon-adc-col-${i}" style="display:none">
                <span class="analog-v-label">${axisNames[i] ? axisNames[i].substring(0, 3).toUpperCase() : 'A' + (i + 1)}</span>
                <div class="analog-v-track">
                    <div class="analog-v-fill" id="mon-adc-fill-${i}"></div>
                </div>
                <span class="analog-v-val" id="mon-adc-value-${i}">0</span>
            </div>
        `).join('');
    }
}

function setupTooltips() {
    document.addEventListener('mouseover', (e) => {
        const target = e.target.closest('[data-help]');
        if (target && elements.tooltip) {
            const helpKey = target.getAttribute('data-help');
            const helpText = translate(helpKey);
            if (helpText && helpText !== helpKey) {
                elements.tooltip.textContent = helpText;
                elements.tooltip.style.display = 'block';
                elements.tooltip.style.opacity = '1';
            }
        }
    });

    document.addEventListener('mousemove', (e) => {
        if (elements.tooltip && elements.tooltip.style.display === 'block') {
            const offsetX = 15;
            const offsetY = 15;
            let left = e.clientX + offsetX;
            let top = e.clientY + offsetY;

            // Prevent tooltip from going off-screen
            const tooltipRect = elements.tooltip.getBoundingClientRect();
            if (left + tooltipRect.width > window.innerWidth) {
                left = e.clientX - tooltipRect.width - offsetX;
            }
            if (top + tooltipRect.height > window.innerHeight) {
                top = e.clientY - tooltipRect.height - offsetY;
            }

            elements.tooltip.style.left = left + 'px';
            elements.tooltip.style.top = top + 'px';
        }
    });

    document.addEventListener('mouseout', (e) => {
        const target = e.target.closest('[data-help]');
        if (target && elements.tooltip) {
            elements.tooltip.style.display = 'none';
            elements.tooltip.style.opacity = '0';
        }
    });
}

function attemptAutoConnect() {
    invoke('connect_hardware').catch(() => { });
}

function showToast(message, type = 'info') {
    if (!elements.toastContainer) return;
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;
    elements.toastContainer.appendChild(toast);
    setTimeout(() => toast.classList.add('visible'), 50);
    setTimeout(() => {
        toast.classList.remove('visible');
        setTimeout(() => toast.remove(), 300);
    }, 2500);
}

function applyTranslations() {
    document.querySelectorAll('[data-i18n]').forEach(el => {
        el.textContent = translate(el.getAttribute('data-i18n'));
    });
    if (elements.tabTitle) elements.tabTitle.textContent = translate(`tab_${currentTab}`);
}

let isKeyboardServiceRunning = false;

function setupEventListeners() {
    elements.navItems.forEach(item => {
        item.addEventListener('click', () => switchTab(item.getAttribute('data-tab')));
    });

    if (elements.btnResetCenter) {
        elements.btnResetCenter.onclick = () => invoke('reset_center').then(() => showToast('Center reset!', 'success'));
    }

    if (elements.btnSave) {
        elements.btnSave.onclick = () => invoke('save_settings').then(() => showToast(translate('msg_rebooting'), 'info'));
    }

    const btnReboot = document.getElementById('btn-reboot');
    if (btnReboot) {
        btnReboot.onclick = () => invoke('reboot_device').then(() => showToast(translate('msg_rebooting'), 'info'));
    }

    const btnStartService = document.getElementById('btn-start-keyboard');
    if (btnStartService) {
        btnStartService.onclick = async () => {
            isKeyboardServiceRunning = !isKeyboardServiceRunning;

            // Sync mappings first
            await sendMappingsToBackend();

            // Call backend toggle
            invoke('toggle_keyboard_service', { enabled: isKeyboardServiceRunning })
                .then(() => {
                    if (isKeyboardServiceRunning) {
                        showToast('Keyboard Service Started', 'success');
                    } else {
                        showToast('Keyboard Service Stopped', 'info');
                    }
                })
                .catch(e => {
                    showToast('Service Error: ' + e, 'error');
                    isKeyboardServiceRunning = false; // Revert
                });

            btnStartService.textContent = isKeyboardServiceRunning ? "Stop Mapping Service" : translate('btn_start_service');
            btnStartService.classList.toggle('active', isKeyboardServiceRunning);
            if (isKeyboardServiceRunning) {
                btnStartService.style.borderColor = 'var(--success)';
                btnStartService.style.color = 'var(--success)';
            } else {
                btnStartService.style.borderColor = '';
                btnStartService.style.color = '';
            }
        };
    }

    // FFB Test Sliders (Live)
    const setupTestFx = (id, type) => {
        const slider = document.getElementById(id);
        if (slider) {
            slider.oninput = (e) => {
                const val = parseInt(e.target.value);
                // Map 0-100 to 0-32767 for sine/damper
                // Constant -100..100 to -32767..32767
                let hwVal = (val / 100) * 32767;
                invoke('send_direct_control', { forceType: type, value: Math.round(hwVal) }).catch(() => { });
            };
            // Reset to 0 on release for safety
            slider.onchange = (e) => {
                e.target.value = 0;
                invoke('send_direct_control', { forceType: type, value: 0 }).catch(() => { });
            };
        }
    };

    setupTestFx('test-fx-constant', 1);
    setupTestFx('test-fx-sine', 2);
    setupTestFx('test-fx-damper', 3);
}

async function sendMappingsToBackend() {
    const mappings = [];
    // axisKeyMapping: { [idx]: { low: 'KEY', high: 'KEY' } }
    for (let i = 0; i < 6; i++) {
        const map = axisKeyMapping[i];
        if (map) {
            if (map.low) {
                mappings.push({
                    id: `axis-${i}-low`,
                    source_type: "axis",
                    index: i,
                    trigger: "low",
                    key: map.low,
                    threshold: 500 // roughly 12%
                });
            }
            if (map.high) {
                mappings.push({
                    id: `axis-${i}-high`,
                    source_type: "axis",
                    index: i,
                    trigger: "high",
                    key: map.high,
                    threshold: 3600 // roughly 88%
                });
            }
        }
    }
    try {
        await invoke('set_keyboard_mapping', { mappings });
        console.log('[Configurator] Mappings synced to backend', mappings);
    } catch (e) {
        console.error('Failed to sync mappings:', e);
        showToast('Mapping Sync Failed', 'error');
    }
}

function setupLicenseUI() {
    const btnActivate = document.getElementById('btn-activate');
    const inputKey = document.getElementById('input-serial-key');
    const btnCopy = document.getElementById('btn-copy-id');

    if (btnActivate && inputKey) {
        btnActivate.onclick = async () => {
            const key = inputKey.value;
            if (!key || key.trim().length < 5) {
                showToast("Please enter a valid License Key.", 'error');
                return;
            }
            try {
                await invoke('activate_license', { keyStr: key });
                showToast("License Activated! Rebooting...", 'success');
                setTimeout(() => loadAllData(), 2000);
            } catch (e) {
                showToast("Activation Failed: " + e, 'error');
            }
        };
    }

    if (btnCopy) {
        btnCopy.onclick = () => {
            const txt = document.getElementById('lic-tab-id')?.textContent || "";
            navigator.clipboard.writeText(txt);
            showToast('ID Copied to clipboard', 'info');
        };
    }
}

function switchTab(tabId) {
    if (currentTab === tabId) return;
    elements.navItems.forEach(item => item.classList.toggle('active', item.getAttribute('data-tab') === tabId));
    elements.tabContents.forEach(content => content.classList.toggle('active', content.id === `tab-${tabId}`));
    if (elements.tabTitle) elements.tabTitle.textContent = translate(`tab_${tabId}`);
    currentTab = tabId;
    loadTabData(tabId);
}

function updateConnectionUI() {
    refreshElementCache();
    console.debug(`[Configurator] UI update: isConnected=${isConnected}`);

    if (elements.connectionStatus) {
        elements.connectionStatus.className = `status-indicator ${isConnected ? 'connected' : 'disconnected'}`;
    }

    if (elements.statusText) {
        elements.statusText.textContent = translate(isConnected ? 'status_connected' : 'status_disconnected');
    }

    if (elements.sysStatus) {
        elements.sysStatus.textContent = translate(isConnected ? 'ready' : 'status_disconnected');
        elements.sysStatus.className = isConnected ? 'stat-val text-green' : 'stat-val text-dim';
    }
}

function updateMonitorVisuals(status) {
    if (!isConnected) return;
    if (!elements.wheelRotateGroup) refreshElementCache();

    // Wheel Rotation
    const range = currentEffectSettings?.motion_range || 900;
    // position is -32768 to +32767, normalize to -1 to +1, then scale by half range
    const angle = (status.position / 32767.0) * (range / 2.0);

    if (elements.wheelRotateGroup) {
        // Apply rotation with proper transform origin (already set in HTML)
        elements.wheelRotateGroup.style.transform = `rotate(${angle}deg)`;
    }
    if (elements.wheelPosValue) {
        elements.wheelPosValue.textContent = `${angle.toFixed(1)}°`;
    }
    // Torque
    if (elements.torqueBarV && status.torque !== undefined) {
        const tVal = Math.abs(status.torque);
        const tPct = Math.min((tVal / 32767) * 100, 100);
        elements.torqueBarV.style.height = `${tPct}%`;
        document.getElementById('torque-value').textContent = `${((status.torque / 32767) * 100).toFixed(1)}%`;
    }

    // Firmware
    if (elements.firmwareVer && status.firmware) {
        elements.firmwareVer.textContent = `v${status.firmware.major}.${status.firmware.minor}.${status.firmware.patch}`;
    }

    // License Info
    const licId = document.getElementById('lic-device-id');
    const licKey = document.getElementById('lic-serial');
    const licStatus = document.getElementById('lic-status');
    const licTabId = document.getElementById('lic-tab-id');
    const licTabStatus = document.getElementById('lic-tab-status');

    // Format helper for ID/Key (hex string)
    const fmtHex = (arr) => {
        if (!arr || !Array.isArray(arr) || arr.length === 0) return '-';
        return arr.map(n => {
            const val = typeof n === 'number' ? n : 0;
            return val.toString(16).toUpperCase().padStart(8, '0');
        }).join('-'); // Use dash to separate 32-bit segments
    };

    const idStr = fmtHex(status.device_id);
    const keyStr = fmtHex(status.serial_key);
    const regText = status.is_registered ? translate("status_activated") : translate("status_trial");
    const regColor = status.is_registered ? "var(--success)" : "var(--warning)";

    if (licId) licId.textContent = idStr;
    if (licKey) licKey.textContent = keyStr;

    if (licStatus) {
        licStatus.textContent = regText;
        licStatus.style.color = regColor;
    }

    if (licTabId) licTabId.textContent = idStr;
    if (licTabStatus) {
        licTabStatus.textContent = regText;
        licTabStatus.style.color = regColor;
    }

    // Buttons Status (32 bitmask)
    if (elements.monitorButtons && status.buttons !== undefined) {
        const bus = status.buttons || 0;
        for (let i = 0; i < 32; i++) {
            const btn = document.getElementById(`mon-btn-${i}`);
            if (btn) {
                // Check if bit i is set in the bitmask
                const active = ((bus >>> i) & 1) === 1;
                btn.classList.toggle('active', active);
            }
        }
    }

    // Analog Status (Vertical Fills)
    if (elements.monitorAnalog && status.adc && Array.isArray(status.adc)) {
        for (let i = 0; i < 6; i++) {
            const col = document.getElementById(`mon-adc-col-${i}`);
            if (!col) continue;

            const val = status.adc[i] || 0;

            // Visibility Logic:
            // 1. Show if it's one of the primary 3 axes (standard for pedals/wheel)
            // 2. Show if it's explicitly set to Analog in Pin Mode
            // 3. Show if it has significant signal (> 20) to help user debug
            let isActive = (i < 3);

            if (currentGpioSettings && currentGpioSettings.pin_mode) {
                if (currentGpioSettings.pin_mode[i] === 2) isActive = true;
            }

            if (!isActive && val > 20) isActive = true;

            if (isActive) {
                col.style.display = 'flex';
                const fill = document.getElementById(`mon-adc-fill-${i}`);
                const text = document.getElementById(`mon-adc-value-${i}`);
                if (fill) {
                    const percentage = Math.min(Math.max((val / 4095) * 100, 0), 100);
                    fill.style.height = `${percentage}%`;
                }
                if (text) text.textContent = val;
            } else {
                col.style.display = 'none';
            }
        }
    }
}

async function loadAllData() {
    console.log('[Configurator] Syncing hardware settings...');
    try {
        const effects = await invoke('get_effect_settings');
        console.log('[Configurator] Effects loaded:', effects);
        currentEffectSettings = effects;

        const hardware = await invoke('get_hardware_settings');
        console.log('[Configurator] Hardware loaded:', hardware);
        currentHardwareSettings = hardware;

        const gpio = await invoke('get_gpio_settings');
        console.log('[Configurator] GPIO loaded:', gpio);
        currentGpioSettings = gpio;

        const adc = await invoke('get_adc_settings');
        console.log('[Configurator] ADC loaded:', adc);
        currentAdcSettings = adc;

        if (!isConnected) {
            isConnected = true;
            updateConnectionUI();
        }

        loadTabData(currentTab);
        console.info('[Configurator] All data synced and tabs updated.');
    } catch (e) {
        console.error('[Configurator] Sync failed:', e);
        showToast('Sync failed: ' + e, 'error');
    }
}

async function loadTabData(tabId) {
    console.log(`[Configurator] Loading tab data for: ${tabId}`);
    if (!isConnected) {
        console.warn(`[Configurator] loadTabData skipped: Not connected`);
        return;
    }
    try {
        switch (tabId) {
            case 'effects': renderEffectsTab(); break;
            case 'hardware': renderHardwareTab(); break;
            case 'protocol': renderProtocolTab(); break;
            case 'pins': renderPinsTab(); break;
            case 'buttons': renderButtonsTab(); break;
            case 'inputs': renderInputsTab(); break;
            case 'tools': renderToolsTab(); break;
            case 'license': /* License tab content is static + auto-updated by telemetry */ break;
            default: console.warn(`[Configurator] No render function for tab: ${tabId}`);
        }
    } catch (e) {
        console.error(`[Configurator] Error rendering tab ${tabId}:`, e);
    }
}

function createSliderRow(item, currentSettings, idPrefix, onSync) {
    const helpKey = `help_${item.key}`;
    const helpIcon = `<img src="https://api.iconify.design/material-symbols:help-outline.svg?color=white" class="help-icon" data-help="${helpKey}">`;

    const html = `
        <div class="setting-row">
            <div class="setting-header">
                <span class="setting-label">${translate(item.label)} ${helpIcon}</span>
                <span class="setting-value" id="val-${idPrefix}-${item.key}">${currentSettings[item.key]}${item.unit}</span>
            </div>
            <div class="setting-control-row" style="display:flex; gap:0.5rem; align-items:center;">
                <input type="range" min="${item.min}" max="${item.max}" step="${item.step}" 
                       value="${currentSettings[item.key]}" data-key="${item.key}" data-unit="${item.unit}"
                       class="setting-slider" id="slider-${idPrefix}-${item.key}" style="flex:1;">
                <input type="number" min="${item.min}" max="${item.max}" step="${item.step}" 
                       value="${currentSettings[item.key]}" 
                       class="setting-input-text" id="input-${idPrefix}-${item.key}" style="width:55px; background:#111; border:1px solid #333; color:white; font-size:0.75rem; padding:2px;">
            </div>
        </div>
    `;
    return html;
}

function attachSliderEvents(container, currentSettings, idPrefix, onSync) {
    container.querySelectorAll('.setting-slider').forEach(slider => {
        const key = slider.getAttribute('data-key');
        const textInput = container.querySelector(`#input-${idPrefix}-${key}`);
        const valDisplay = container.querySelector(`#val-${idPrefix}-${key}`);
        const unit = slider.getAttribute('data-unit');

        const update = (val) => {
            val = Math.min(Math.max(val, slider.min), slider.max);
            slider.value = val;
            textInput.value = val;
            if (valDisplay) valDisplay.textContent = `${val}${unit}`;
            currentSettings[key] = val;
        };

        slider.oninput = (e) => update(parseInt(e.target.value));
        textInput.oninput = (e) => update(parseInt(e.target.value) || 0);
        slider.onchange = onSync;
        textInput.onchange = onSync;
    });
}

function renderHardwareTab() {
    const container = document.querySelector('#tab-hardware .settings-grid');
    if (!container || !currentHardwareSettings) return;
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
        ].map(i => createSliderRow(i, currentHardwareSettings, 'hw')).join('')}
        </div>
        <div class="setting-group card">
            <h3>${translate('group_pid')}</h3>
            ${[
            { key: 'proportional_gain', label: 'setting_p_gain', min: 0, max: 255, step: 1, unit: '' },
            { key: 'integral_gain', label: 'setting_i_gain', min: 0, max: 2000, step: 1, unit: '' }
        ].map(i => createSliderRow(i, currentHardwareSettings, 'hw')).join('')}
        </div>
        <div class="setting-group card">
            <h3>${translate('group_maintenance')}</h3>
            <button class="btn primary" style="background:var(--warning); color:black; width:100%;" id="btn-enter-dfu">
                ${translate('btn_enter_dfu')}
            </button>
        </div>
    `;
    attachSliderEvents(container, currentHardwareSettings, 'hw', () => invoke('update_hardware_settings', { settings: currentHardwareSettings }));

    const btnDfu = document.getElementById('btn-enter-dfu');
    if (btnDfu) {
        btnDfu.onclick = async () => {
            if (confirm(translate('confirm_dfu'))) {
                try {
                    await invoke('switch_to_dfu');
                    showToast('Device switched to DFU mode', 'success');
                } catch (e) {
                    showToast('DFU Switch failed: ' + e, 'error');
                }
            }
        };
    }
}

function renderEffectsTab() {
    const container = document.querySelector('#tab-effects .settings-grid');
    if (!container || !currentEffectSettings) return;
    container.innerHTML = `
        <div class="setting-group card">
            <h3>${translate('group_general')}</h3>
            ${[
            { key: 'motion_range', label: 'setting_motion_range', min: 90, max: 1080, step: 1, unit: '°' },
            { key: 'total_effect_strength', label: 'setting_total_strength', min: 0, max: 100, step: 1, unit: '%' },
            { key: 'integrated_spring_strength', label: 'setting_integrated_spring', min: 0, max: 255, step: 1, unit: '' }
        ].map(i => createSliderRow(i, currentEffectSettings, 'fx')).join('')}
        </div>
        <div class="setting-group card">
            <h3>${translate('group_dampening')}</h3>
            ${[
            { key: 'static_dampening_strength', label: 'setting_static_dampening', min: 0, max: 1000, step: 1, unit: '' },
            { key: 'dynamic_dampening_strength', label: 'setting_dynamic_dampening', min: 0, max: 1000, step: 1, unit: '' }
        ].map(i => createSliderRow(i, currentEffectSettings, 'fx')).join('')}
        </div>
        <div class="setting-group card">
            <h3>${translate('group_direct_x')}</h3>
            ${[
            { key: 'direct_x_constant_strength', label: 'setting_dx_constant', min: 0, max: 100, step: 1, unit: '%' },
            { key: 'direct_x_periodic_strength', label: 'setting_dx_periodic', min: 0, max: 100, step: 1, unit: '%' },
            { key: 'direct_x_spring_strength', label: 'setting_dx_spring', min: 0, max: 100, step: 1, unit: '%' }
        ].map(i => createSliderRow(i, currentEffectSettings, 'fx')).join('')}
        </div>
    `;
    attachSliderEvents(container, currentEffectSettings, 'fx', () => invoke('update_effect_settings', { settings: currentEffectSettings }));
}

function renderProtocolTab() {
    const container = document.querySelector('#tab-protocol .settings-grid');
    const helpIcon = `<img src="https://api.iconify.design/material-symbols:help-outline.svg?color=white" class="help-icon" data-help="help_ext_mode">`;

    const currentMode = currentGpioSettings.extension_mode || 0;

    // Key mappings matching en.json/pt.json
    const titleKeys = ['mode_none', 'mode_buttons', 'mode_spi_tm', 'mode_spi_fanatec'];
    const detailKeys = ['none', 'buttons', 'tm', 'fanatec'];

    const titleKey = titleKeys[currentMode] || 'mode_none';
    const detailKey = detailKeys[currentMode] || 'none';

    container.innerHTML = `
        <div class="card wide">
            <h3>${translate('group_extension')}</h3>
            <div class="setting-row">
                <span class="setting-label">${translate('setting_ext_mode')} ${helpIcon}</span>
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
        invoke('update_gpio_settings', { settings: currentGpioSettings });
        renderProtocolTab(); // Re-render to update info box
    };
}

function renderPinsTab() {
    const container = document.querySelector('#tab-pins .settings-grid');
    const pinModes = ["none", "gpio", "analog", "spi_cs", "spi_sck", "spi_miso", "enable_effects", "center_reset", "braking_pwm", "effect_led", "reboot"];
    container.innerHTML = `
        <div class="card wide">
            <h3>${translate('group_pins')}</h3>
            <div style="display:grid; grid-template-columns: 1fr 1fr; gap: 8px 15px;">
                ${currentGpioSettings.pin_mode.map((mode, i) => `
                    <div class="setting-row" style="margin-bottom:0;">
                        <span class="setting-label" style="font-size:0.85rem; color:var(--text-dim); margin-bottom:2px;">${translate('pin_label')} ${i}</span>
                        <select class="pin-sel" data-idx="${i}" style="background:#111; border:1px solid #333; border-radius:4px; color:white; width:100%; font-size:0.85rem; padding:4px;">
                            ${pinModes.map((m, idx) => `<option value="${idx}" ${mode === idx ? 'selected' : ''}>${translate('pin_mode_' + m)}</option>`).join('')}
                        </select>
                    </div>
                `).join('')}
            </div>
        </div>
    `;
    container.querySelectorAll('.pin-sel').forEach(s => s.onchange = (e) => {
        currentGpioSettings.pin_mode[parseInt(e.target.dataset.idx)] = parseInt(e.target.value);
        invoke('update_gpio_settings', { settings: currentGpioSettings });
    });
}

function renderButtonsTab() {
    const container = document.querySelector('#tab-buttons .settings-grid');
    const btnModes = ["none", "normal", "inverted", "pulse"];
    container.innerHTML = `
        <div class="card wide">
            <h3>${translate('group_buttons')}</h3>
            <div style="display:grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 8px;">
                ${currentGpioSettings.button_mode.slice(0, 32).map((mode, i) => `
                    <div class="setting-row vertical">
                        <span class="setting-label" style="font-size:0.85rem; color:var(--text-dim);">${translate('btn_label')} ${i + 1}</span>
                        <select class="btn-sel" data-idx="${i}" style="background:#111; border:1px solid #333; border-radius:4px; color:white; width:100%; font-size:0.85rem; padding:4px;">
                            ${btnModes.map((m, idx) => `<option value="${idx}" ${mode === idx ? 'selected' : ''}>${translate('btn_mode_' + m)}</option>`).join('')}
                        </select>
                    </div>
                `).join('')}
            </div>
        </div>
    `;
    container.querySelectorAll('.btn-sel').forEach(s => s.onchange = (e) => {
        currentGpioSettings.button_mode[parseInt(e.target.dataset.idx)] = parseInt(e.target.value);
        invoke('update_gpio_settings', { settings: currentGpioSettings });
    });
}

let currentMappingIdx = -1;

function openMappingModal(idx) {
    currentMappingIdx = idx;
    const modal = document.getElementById('modal-axis-mapping');
    const inputName = document.getElementById('modal-axis-name');
    const btnLow = document.getElementById('modal-btn-low');
    const btnHigh = document.getElementById('modal-btn-high');
    const keyLow = document.getElementById('modal-key-low');
    const keyHigh = document.getElementById('modal-key-high');

    // Fill Name
    inputName.value = axisNames[idx] || '';

    // Populate Buttons
    const btnOptions = `<option value="0">${translate('btn_none')}</option>` +
        Array.from({ length: 32 }, (_, i) => `<option value="${i + 1}">${translate('btn_label')} ${i + 1}</option>`).join('');
    btnLow.innerHTML = btnOptions;
    btnHigh.innerHTML = btnOptions;

    btnLow.value = currentAdcSettings.raxis_to_button_low[idx] || 0;
    btnHigh.value = currentAdcSettings.raxis_to_button_high[idx] || 0;

    // Populate Keys
    const keyOptions = `<option value="">${translate('btn_none')}</option>` +
        KEY_OPTIONS.map(k => `<option value="${k}">${k}</option>`).join('');
    keyLow.innerHTML = keyOptions;
    keyHigh.innerHTML = keyOptions;

    const keyMap = axisKeyMapping[idx] || {};
    keyLow.value = keyMap.low || '';
    keyHigh.value = keyMap.high || '';

    modal.classList.add('active');

    // Attach Save/Close handlers dynamically to capture current scope/idx
    document.getElementById('modal-save').onclick = () => {
        if (currentMappingIdx === -1) return;

        // Save Name
        axisNames[currentMappingIdx] = inputName.value;
        localStorage.setItem('ffbeast_axis_names', JSON.stringify(axisNames));

        // Save Digital Mapping (Only if supported by hardware struct - first 3)
        if (currentMappingIdx < 3) {
            currentAdcSettings.raxis_to_button_low[currentMappingIdx] = parseInt(btnLow.value);
            currentAdcSettings.raxis_to_button_high[currentMappingIdx] = parseInt(btnHigh.value);
            invoke('update_adc_settings', { settings: currentAdcSettings });
        }

        // Save Key Mapping
        axisKeyMapping[currentMappingIdx] = {
            low: keyLow.value,
            high: keyHigh.value
        };
        localStorage.setItem('ffbeast_axis_keys', JSON.stringify(axisKeyMapping));

        showToast(translate('btn_save'), 'success');

        // Refresh UI
        initMonitorVisuals(); // Update names in monitor
        renderInputsTab();    // Update names in list
        closeMappingModal();
    };

    document.getElementById('modal-close').onclick = closeMappingModal;
}

function closeMappingModal() {
    document.getElementById('modal-axis-mapping').classList.remove('active');
    currentMappingIdx = -1;
}

function renderInputsTab() {
    const container = document.querySelector('#tab-inputs .settings-grid');
    if (!container || !currentAdcSettings) return;

    // Determine which axes to show: first 3 are always shown, others only if configured as Analog
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
                    <button class="btn-outline edit-mapping-btn" data-idx="${i}" style="font-size:0.7rem;">
                        <img src="https://api.iconify.design/material-symbols:edit-outline.svg?color=white" style="width:14px; vertical-align:middle; margin-right:4px;">
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
        btn.onclick = (e) => openMappingModal(parseInt(e.currentTarget.dataset.idx));
    });

    activeIndices.forEach(i => {
        if (i < 3) {
            const valSet = { raxis_min: currentAdcSettings.raxis_min[i], raxis_max: currentAdcSettings.raxis_max[i] };
            attachSliderEvents(container.querySelector(`#adc-config-${i}`), valSet, `adc-${i}`, () => {
                currentAdcSettings.raxis_min[i] = valSet.raxis_min;
                currentAdcSettings.raxis_max[i] = valSet.raxis_max;
                invoke('update_adc_settings', { settings: currentAdcSettings });
            });
        }
    });
}

function renderToolsTab() {
    const container = document.querySelector('#tab-tools .settings-grid');
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

            invoke('send_direct_control', { forceType, value: Math.round(hwVal) }).catch(() => { });
        });

        // Reset on release
        forceSlider.addEventListener('change', (e) => {
            e.target.value = 0;
            forceValue.textContent = '0%';
            invoke('send_direct_control', { forceType: 1, value: 0 }).catch(() => { });
        });
    }

    // DFU button
    const dfuBtn = container.querySelector('#btn-dfu-mode');
    if (dfuBtn) {
        dfuBtn.addEventListener('click', async () => {
            if (confirm(translate('confirm_dfu'))) {
                try {
                    await invoke('switch_to_dfu');
                    showToast(translate('toast_dfu_success'), 'success');
                } catch (e) {
                    showToast(translate('toast_dfu_failed') + ': ' + e, 'error');
                }
            }
        });
    }
}

init();
