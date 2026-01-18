import { translate } from '../services/i18n.js';

export function createSliderRow(item, currentSettings, idPrefix) {
    const helpKey = `help_${item.key}`;
    const helpIcon = `<img src="./assets/icons/help.svg" class="help-icon" data-help="${helpKey}">`;

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

export function attachSliderEvents(container, currentSettings, idPrefix, onSync) {
    container.querySelectorAll('.setting-slider').forEach(slider => {
        const key = slider.getAttribute('data-key');
        const textInput = container.querySelector(`#input-${idPrefix}-${key}`);
        const valDisplay = container.querySelector(`#val-${idPrefix}-${key}`);
        const unit = slider.getAttribute('data-unit');
        const min = parseFloat(slider.min); // Use parseFloat in case of decimals
        const max = parseFloat(slider.max);

        // Slider -> Text Input (Immediate)
        slider.oninput = (e) => {
            const val = parseFloat(e.target.value);
            textInput.value = val;
            if (valDisplay) valDisplay.textContent = `${val}${unit}`;
            currentSettings[key] = val;
        };
        slider.onchange = onSync; // Save when slider released

        // Text Input -> Slider (Debounced)
        let debounceTimer;

        const validateAndSync = (finalVal) => {
            if (isNaN(finalVal)) finalVal = min;
            if (finalVal < min) finalVal = min;
            if (finalVal > max) finalVal = max;

            textInput.value = finalVal;
            slider.value = finalVal;
            if (valDisplay) valDisplay.textContent = `${finalVal}${unit}`;
            currentSettings[key] = finalVal;
            onSync();
        };

        textInput.oninput = (e) => {
            const rawVal = e.target.value;
            clearTimeout(debounceTimer);

            // Wait for user to stop typing
            debounceTimer = setTimeout(() => {
                let val = parseFloat(rawVal);
                // Don't sync yet if invalid or explicitly typing
                if (isNaN(val)) return;
                // Validate range but don't save yet? 
                // Actually user asked for delay validation.
                validateAndSync(val);
            }, 800);
        };

        textInput.onblur = (e) => {
            clearTimeout(debounceTimer);
            validateAndSync(parseFloat(e.target.value));
        };

        textInput.onkeydown = (e) => {
            if (e.key === 'Enter') textInput.blur();
        };
    });
}

export function createCheckboxRow(item, currentSettings, idPrefix) {
    const helpKey = `help_${item.key}`;
    const helpIcon = `<img src="./assets/icons/help.svg" class="help-icon" data-help="${helpKey}">`;

    // Determine checked state based on trueValue (default 1)
    const trueVal = item.trueValue !== undefined ? item.trueValue : 1;
    const falseVal = item.falseValue !== undefined ? item.falseValue : 0;

    // Check if current value matches trueVal (loose equality for 1 vs true?)
    // Strict equality is safer if types match
    const isChecked = currentSettings[item.key] === trueVal ? 'checked' : '';

    return `
        <div class="setting-row" style="display:flex; justify-content:space-between; align-items:center; min-height: 40px;">
             <span class="setting-label">${translate(item.label)} ${helpIcon}</span>
             <label class="switch" style="position:relative; display:inline-block; width:40px; height:20px;">
                <input type="checkbox" id="chk-${idPrefix}-${item.key}" 
                       data-key="${item.key}" 
                       data-true="${trueVal}" 
                       data-false="${falseVal}" 
                       ${isChecked}
                       style="opacity:0; width:0; height:0;">
                <span class="slider round" style="position:absolute; cursor:pointer; top:0; left:0; right:0; bottom:0; background-color:#333; transition:.4s; border-radius:34px;"></span>
            </label>
            <style>
                .switch input:checked + .slider { background-color: var(--accent); }
                .switch input:focus + .slider { box-shadow: 0 0 1px var(--accent); }
                .switch input:checked + .slider:before { transform: translateX(20px); }
                .slider:before { position: absolute; content: ""; height: 16px; width: 16px; left: 2px; bottom: 2px; background-color: white; transition: .4s; border-radius: 50%; }
            </style>
        </div>
    `;
}

export function attachCheckboxEvents(container, currentSettings, idPrefix, onSync) {
    container.querySelectorAll(`input[type="checkbox"][id^="chk-${idPrefix}-"]`).forEach(chk => {
        const key = chk.getAttribute('data-key');
        const trueVal = parseFloat(chk.getAttribute('data-true'));
        const falseVal = parseFloat(chk.getAttribute('data-false'));

        chk.onchange = (e) => {
            currentSettings[key] = e.target.checked ? trueVal : falseVal;
            onSync();
        };
    });
}
