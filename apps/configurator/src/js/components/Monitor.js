import { translate } from '../services/i18n.js';

export class MonitorComponent {
    constructor(elements) {
        this.elements = elements;
        this.firstPacketLogged = false;
    }

    init(axisNames) {
        if (this.elements.monitorButtons) {
            this.elements.monitorButtons.innerHTML = Array.from({ length: 32 }, (_, i) => `
                <div class="btn-indicator" id="mon-btn-${i}">${i + 1}</div>
            `).join('');
        }
        if (this.elements.monitorAnalog) {
            this.elements.monitorAnalog.innerHTML = [0, 1, 2, 3, 4, 5].map(i => `
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

    update(status, currentGpioSettings, currentEffectSettings) {
        if (!status) {
            console.warn('[Monitor] update() called with null status');
            return;
        }

        if (!this.firstPacketLogged) {
            console.log('[Monitor] First status update:', {
                position: status.position,
                buttons: status.buttons,
                buttonsType: typeof status.buttons,
                adcCount: status.adc?.length,
                adcValues: status.adc,
                adcType: typeof status.adc,
                hasEffectSettings: !!currentEffectSettings,
                motionRange: currentEffectSettings?.motion_range,
                hasGpioSettings: !!currentGpioSettings
            });
            this.firstPacketLogged = true;
        }

        // Wheel Rotation - use motion_range from effect settings
        if (this.elements.wheelRotateGroup) {
            const range = currentEffectSettings?.motion_range || 900;
            // CORRECT VALUE: 10000. (User confirmed 32767 gives 27deg for 90deg input, which is ~0.3 ratio)
            const angle = (status.position / 10000.0) * (range / 2.0);
            this.elements.wheelRotateGroup.style.transform = `rotate(${angle}deg)`;
            if (Math.abs(angle) > 10) {
                console.log('[Monitor] Wheel:', { pos: status.position, range, angle: angle.toFixed(1) });
            }
        } else {
            console.error('[Monitor] wheelRotateGroup element NOT FOUND!');
        }
        if (this.elements.wheelPosValue) {
            const range = currentEffectSettings?.motion_range || 900;
            const angle = (status.position / 10000.0) * (range / 2.0);
            this.elements.wheelPosValue.textContent = `${angle.toFixed(1)}°`;
        } else {
            console.error('[Monitor] wheelPosValue element NOT FOUND!');
        }

        // Torque Bar - scale to 10000 as per user feedback
        if (this.elements.torqueBarV) {
            const torquePercent = Math.min(100, (Math.abs(status.torque) / 10000.0) * 100);
            this.elements.torqueBarV.style.height = `${torquePercent}%`;
        }
        if (this.elements.torqueValueNode) {
            this.elements.torqueValueNode.textContent = status.torque;
        }

        // Buttons - check each bit in the 32-bit mask
        if (status.buttons !== undefined) {
            const buttons = status.buttons || 0;
            let activeButtons = [];
            for (let i = 0; i < 32; i++) {
                const btn = document.getElementById(`mon-btn-${i}`);
                if (btn) {
                    const isActive = ((buttons >>> i) & 1) === 1;
                    btn.classList.toggle('active', isActive);
                    if (isActive) activeButtons.push(i + 1);
                } else if (i === 0) {
                    console.error('[Monitor] Button element mon-btn-0 NOT FOUND!');
                }
            }
            if (activeButtons.length > 0) {
                console.log('[Monitor] Active buttons:', activeButtons.join(', '));
            }

            // Log button state periodically even when zero
            if (!this.buttonLogCount) this.buttonLogCount = 0;
            this.buttonLogCount++;
            if (this.buttonLogCount % 100 === 0) {
                console.log('[Monitor] Button state check:', {
                    buttons: buttons,
                    activeCount: activeButtons.length,
                    binary: buttons.toString(2).padStart(32, '0')
                });
            }
        } else {
            console.warn('[Monitor] status.buttons is undefined');
        }

        // Analog Status - vertical bars for ADC inputs
        if (this.elements.monitorAnalog && status.adc && Array.isArray(status.adc)) {
            let activeAxes = [];
            let allValues = [];
            for (let i = 0; i < 6; i++) {
                const col = document.getElementById(`mon-adc-col-${i}`);
                if (!col) {
                    if (i === 0) console.error('[Monitor] Analog column mon-adc-col-0 NOT FOUND!');
                    continue;
                }

                const val = status.adc[i] || 0;
                allValues.push(val);

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
                        // ADC is 12-bit (0-4095), clamp percentage to 0-100%
                        const percentage = Math.min(Math.max((val / 4095) * 100, 0), 100);
                        fill.style.height = `${percentage}%`;
                    }
                    if (text) text.textContent = val;
                    if (val > 100) activeAxes.push(`A${i}:${val}`);
                } else {
                    col.style.display = 'none';
                }
            }
            if (activeAxes.length > 0) {
                console.log('[Monitor] Active analog axes:', activeAxes.join(', '));
            }

            // Log ADC state periodically even when zero
            if (!this.adcLogCount) this.adcLogCount = 0;
            this.adcLogCount++;
            if (this.adcLogCount % 100 === 0) {
                console.log('[Monitor] ADC state check:', {
                    values: allValues,
                    nonZero: allValues.filter(v => v > 0).length
                });
            }
        } else {
            if (!this.elements.monitorAnalog) console.error('[Monitor] monitorAnalog element NOT FOUND!');
            if (!status.adc) console.warn('[Monitor] status.adc is undefined');
        }

        // Firmware & License
        if (this.elements.firmwareVer && status.firmware) {
            this.elements.firmwareVer.textContent = `v${status.firmware.major}.${status.firmware.minor}.${status.firmware.patch}`;
        }

        this.updateLicenseInfo(status);
    }

    updateLicenseInfo(status) {
        const licId = document.getElementById('lic-device-id');
        const licKey = document.getElementById('lic-serial');
        const licStatus = document.getElementById('lic-status');

        const fmtHex = (arr) => {
            if (!arr || !Array.isArray(arr) || arr.length === 0) return '-';
            return arr.map(n => {
                const val = typeof n === 'number' ? n : 0;
                return val.toString(16).toUpperCase().padStart(8, '0');
            }).join('-');
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

        const tabLicId = document.getElementById('lic-tab-id');
        const tabLicStatus = document.getElementById('lic-tab-status');
        if (tabLicId) tabLicId.textContent = idStr;
        if (tabLicStatus) {
            tabLicStatus.textContent = regText;
            tabLicStatus.className = `status-badge ${status.is_registered ? 'success' : 'warning'}`;
        }
    }
}
