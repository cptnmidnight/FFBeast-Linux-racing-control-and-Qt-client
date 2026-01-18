import { loadTranslations, detectLanguage, applyTranslations, translate } from './services/i18n.js';
import { HardwareService } from './services/HardwareService.js';
import { refreshElementCache } from './utils/dom.js';
import { showToast, setupTooltips } from './utils/ui.js';

import { MonitorComponent } from './components/Monitor.js';
import { HardwareTab } from './components/HardwareTab.js';
import { EffectsTab } from './components/EffectsTab.js';
import { PinsTab } from './components/PinsTab.js';
import { ButtonsTab } from './components/ButtonsTab.js';
import { ProtocolTab } from './components/ProtocolTab.js';
import { InputsTab } from './components/InputsTab.js';
import { ToolsTab } from './components/ToolsTab.js';
import { SettingsTab } from './components/SettingsTab.js';
import { LicenseTab } from './components/LicenseTab.js';
import { ModalManager } from './components/ModalManager.js';
import { Logger } from './utils/Logger.js';

class App {
    constructor() {
        this.currentTab = 'monitor';
        this.isConnected = false;
        this.language = detectLanguage();
        this.elements = {};
        this.monitor = null;
        this.modalManager = null;

        // Settings State
        this.currentEffectSettings = null;
        this.currentHardwareSettings = null;
        this.currentGpioSettings = null;
        this.currentAdcSettings = null;

        // Axis Persistence
        const savedNames = JSON.parse(localStorage.getItem('ffbeast_axis_names') || '[]');
        const defaultNames = ["Throttle", "Brake", "Clutch", "A4", "A5", "A6"];
        this.axisNames = defaultNames.map((name, i) => savedNames[i] || name);
        this.axisKeyMapping = JSON.parse(localStorage.getItem('ffbeast_axis_keys') || '[]');
        while (this.axisKeyMapping.length < 6) this.axisKeyMapping.push({});

        this.keyOptions = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
            "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
            "Up", "Down", "Left", "Right", "Space", "Enter", "Tab", "Shift", "Ctrl", "Alt", "Esc"
        ];
    }

    async init() {
        console.info('[App] Initializing modular architecture...');

        Logger.init();
        Logger.subscribe(entry => {
            const panels = [document.getElementById('monitor-log-panel'), document.getElementById('logs-tab-panel')];
            const autoScroll = document.getElementById('chk-auto-scroll')?.checked ?? true;
            panels.forEach(p => {
                if (p) {
                    const el = Logger.renderEntry(entry);
                    p.appendChild(el);

                    // DOM Performance: Limit visible logs
                    while (p.children.length > 200) {
                        p.removeChild(p.firstChild);
                    }

                    if (autoScroll) p.scrollTop = p.scrollHeight;
                }
            });
        });
        SettingsTab.applySavedSettings();

        await loadTranslations(this.language);
        this.elements = refreshElementCache();
        console.log('[App] Elements cached:', Object.keys(this.elements).filter(k => this.elements[k]).length, '/', Object.keys(this.elements).length);

        applyTranslations();
        setupTooltips(this.elements);

        this.monitor = new MonitorComponent(this.elements);
        this.monitor.init(this.axisNames);
        console.log('[App] Monitor component initialized');

        this.modalManager = new ModalManager(
            this.elements, this.axisNames, this.axisKeyMapping,
            null, this.keyOptions
        );

        this.setupEventListeners();

        // Load initial data BEFORE setting up status updates
        // This ensures currentEffectSettings is available for wheel rotation
        console.log('[App] Loading initial data...');
        await this.loadAllData();
        console.log('[App] Initial data loaded. currentEffectSettings:', this.currentEffectSettings);

        this.startBackgroundChecks();
        console.log('[App] Background checks started');

        HardwareService.onStatusUpdate((status) => {
            if (!this.isConnected) {
                console.info('[App] Hardware connected via telemetry');
                this.isConnected = true;
                this.updateConnectionUI();
            }
            // Now currentEffectSettings is guaranteed to be loaded
            this.monitor.update(status, this.currentGpioSettings, this.currentEffectSettings);
        });

        console.log('[App] Initialization complete!');
    }

    setupEventListeners() {
        this.elements.navItems.forEach(item => {
            item.addEventListener('click', () => this.switchTab(item.getAttribute('data-tab')));
        });

        this.elements.btnResetCenter.onclick = () =>
            HardwareService.resetCenter().then(() => showToast(this.elements, translate('toast_center_reset'), 'success'));

        this.elements.btnSave.onclick = () =>
            HardwareService.save().then(() => showToast(this.elements, translate('msg_rebooting'), 'info'));

        const btnReboot = document.getElementById('btn-reboot');
        if (btnReboot) {
            btnReboot.onclick = () =>
                HardwareService.reboot().then(() => showToast(this.elements, translate('msg_rebooting'), 'info'));
        }


        // FFB Test Sliders
        const setupTestFx = (id, type) => {
            const slider = document.getElementById(id);
            if (slider) {
                slider.oninput = (e) => {
                    const val = parseInt(e.target.value);
                    let hwVal = (val / 100) * 32767;
                    HardwareService.sendDirectControl(type, Math.round(hwVal)).catch(() => { });
                };
                slider.onchange = (e) => {
                    e.target.value = 0;
                    HardwareService.sendDirectControl(type, 0).catch(() => { });
                };
            }
        };
        setupTestFx('test-fx-damper', 3);

        // Keyboard Service
        const btnStartService = document.getElementById('btn-start-keyboard');
        if (btnStartService) {
            btnStartService.onclick = async () => {
                const isActive = btnStartService.classList.contains('active');
                Logger.init();
                console.info(`[Service] ${isActive ? 'Stopping' : 'Starting'} Keyboard Service...`);
                try {
                    await HardwareService.toggleKeyboardService(!isActive);
                    btnStartService.classList.toggle('active', !isActive);
                    btnStartService.textContent = !isActive ? translate('btn_stop_service') || 'Stop Service' : translate('btn_start_service') || 'Start Mapping Service';
                    if (!isActive) btnStartService.classList.replace('btn-outline', 'btn-primary');
                    else btnStartService.classList.replace('btn-primary', 'btn-outline');

                    showToast(this.elements, translate(!isActive ? 'toast_service_started' : 'toast_service_stopped'), 'success');
                } catch (e) {
                    console.error('[Service] Failed to toggle service:', e);
                    showToast(this.elements, translate('toast_service_error'), 'error');
                }
            };
        }
    }

    async switchTab(tabId) {
        if (this.currentTab === tabId) return;
        this.currentTab = tabId;

        this.elements.navItems.forEach(item => item.classList.toggle('active', item.getAttribute('data-tab') === tabId));
        this.elements.tabContents.forEach(content => content.classList.toggle('active', content.id === `tab-${tabId}`));

        if (this.elements.tabTitle) this.elements.tabTitle.textContent = translate(`tab_${tabId}`);

        // Update Header Actions
        const defaultActions = document.getElementById('default-actions');
        const logActions = document.getElementById('log-actions');
        if (defaultActions) defaultActions.style.display = tabId === 'logs' ? 'none' : 'flex';
        if (logActions) logActions.style.display = tabId === 'logs' ? 'flex' : 'none';

        if (this.isConnected) {
            if (!this.currentHardwareSettings || !this.currentGpioSettings) {
                await this.loadAllData();
            } else {
                this.loadTabData(tabId);
            }
        }
    }

    async loadAllData() {
        console.log('[App] loadAllData() called');
        try {
            console.log('[App] Fetching hardware settings...');
            this.currentHardwareSettings = await HardwareService.getHardwareSettings();
            console.log('[App] Hardware settings loaded:', this.currentHardwareSettings);

            console.log('[App] Fetching GPIO settings...');
            this.currentGpioSettings = await HardwareService.getGpioSettings();
            console.log('[App] GPIO settings loaded');

            console.log('[App] Fetching ADC settings...');
            this.currentAdcSettings = await HardwareService.getAdcSettings();
            console.log('[App] ADC settings loaded');

            console.log('[App] Fetching effect settings...');
            this.currentEffectSettings = await HardwareService.getEffectSettings();
            console.log('[App] Effect settings loaded:', {
                motion_range: this.currentEffectSettings?.motion_range,
                total_effect_strength: this.currentEffectSettings?.total_effect_strength
            });

            // Update modal manager's ADC reference
            this.modalManager.currentAdcSettings = this.currentAdcSettings;

            if (!this.isConnected) {
                console.log('[App] Marking as connected after successful data load');
                this.isConnected = true;
                this.updateConnectionUI();
            }

            console.log('[App] State synced successfully');
            this.loadTabData(this.currentTab);
        } catch (e) {
            console.error('[App] Initial sync failed:', e);
        }
    }

    loadTabData(tabId) {
        const grid = document.querySelector(`#tab-${tabId} .settings-grid`);
        switch (tabId) {
            case 'hardware': HardwareTab.render(grid, this.currentHardwareSettings, this.elements); break;
            case 'effects': EffectsTab.render(grid, this.currentEffectSettings); break;
            case 'pins': PinsTab.render(grid, this.currentGpioSettings); break;
            case 'buttons': ButtonsTab.render(grid, this.currentGpioSettings); break;
            case 'protocol': ProtocolTab.render(grid, this.currentGpioSettings); break;
            case 'tools': ToolsTab.render(grid); break;
            case 'settings': SettingsTab.render(grid); break;
            case 'license': LicenseTab.render(grid, this.elements); break;
            case 'logs': /* Logs are auto-handled by Logger subscriber */ break;
            case 'inputs':
                InputsTab.render(grid, this.currentAdcSettings, this.currentGpioSettings, this.axisNames,
                    (idx) => this.modalManager.openMappingModal(idx, () => {
                        this.monitor.init(this.axisNames);
                        this.loadTabData('inputs');
                    })
                );
                break;
        }
    }

    updateConnectionUI() {
        if (this.elements.connectionStatus) {
            this.elements.connectionStatus.className = `status-indicator ${this.isConnected ? 'connected' : 'disconnected'}`;
        }
        if (this.elements.statusText) {
            this.elements.statusText.textContent = translate(this.isConnected ? 'status_connected' : 'status_disconnected');
        }
        if (this.elements.sysStatus) {
            this.elements.sysStatus.textContent = translate(this.isConnected ? 'ready' : 'status_disconnected');
            this.elements.sysStatus.className = this.isConnected ? 'stat-val text-green' : 'stat-val text-dim';
        }
    }

    startBackgroundChecks() {
        setInterval(async () => {
            const connected = await HardwareService.isConnected();
            if (connected !== this.isConnected) {
                console.info(`[App] Connection toggle: ${connected}`);
                this.isConnected = connected;
                this.updateConnectionUI();
                if (this.isConnected) this.loadAllData();
            }
            if (!this.isConnected) HardwareService.connect().catch(() => { });
        }, 2000);
    }
}

const app = new App();
window.addEventListener('DOMContentLoaded', () => app.init());
