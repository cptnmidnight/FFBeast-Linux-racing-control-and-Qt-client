<template>
  <div class="inputs-tab">
    <div class="axes-grid">
      <div v-for="index in activeIndices" :key="index" class="axis-card">
        <div class="axis-header">
          <div class="axis-info">
            <span class="axis-label">{{ getAxisLabel(index) }}</span>
            <input 
              type="text" 
              v-model="axisNames[index]" 
              class="axis-name-input"
              :placeholder="DEFAULT_NAMES[index]"
              @blur="saveAxisName"
            />
          </div>
          <button class="icon-btn" @click="editMapping(index)" :data-help="'help_edit_mapping'">⚙️ {{ $t('btn_edit_mapping') }}</button>
        </div>
        
        <div class="adc-config-section" v-if="index < 3">
          <BaseSlider 
            :model-value="store.adc?.raxis_min[index] ?? 0" 
            :label="$t('setting_min')" 
            :max="32767" 
            @update:model-value="v => updateMin(index, v)"
          />
          <BaseSlider 
            :model-value="store.adc?.raxis_max[index] ?? 32767" 
            :label="$t('setting_max')" 
            :max="32767" 
            @update:model-value="v => updateMax(index, v)"
          />
          <BaseSwitch 
            :model-value="store.adc?.raxis_invert[index] === 1" 
            :label="$t('setting_axis_invert')" 
            :help="$t('help_axis_invert')"
            @update:model-value="v => updateInvert(index, v)"
          />
        </div>
        <div v-else class="adc-info">
          <p>Hardware calibration only for primary axes.</p>
        </div>
      </div>
    </div>

    <!-- Mapping Modal -->
    <BaseModal 
      :show="showModal" 
      :title="$t('axis_edit_title') + ' ' + (editingIdx !== null ? (editingIdx + 1) : '')"
      @close="closeModal"
    >
      <div v-if="editingIdx !== null" class="modal-form">
        <div class="form-group">
          <label>{{ $t('axis_custom_name') }}</label>
          <input type="text" v-model="axisNames[editingIdx]" class="base-input">
        </div>

        <div class="section-divider">{{ $t('axis_joystick_mapping') }}</div>
        <div class="form-row">
          <BaseSelect 
            v-model="mappings[editingIdx].btnLow" 
            :options="buttonOptions" 
            :label="$t('axis_button_low')"
            :use-i18n="true"
          />
          <BaseSelect 
            v-model="mappings[editingIdx].btnHigh" 
            :options="buttonOptions" 
            :label="$t('axis_button_high')"
            :use-i18n="true"
          />
        </div>

        <div class="section-divider">{{ $t('axis_keyboard_mapping') }}</div>
        <div class="form-row">
          <BaseSelect 
            v-model="mappings[editingIdx].keyLow" 
            :options="keyOptions" 
            :label="$t('axis_key_low')"
            :use-i18n="true"
          />
          <BaseSelect 
            v-model="mappings[editingIdx].keyHigh" 
            :options="keyOptions" 
            :label="$t('axis_key_high')"
            :use-i18n="true"
          />
        </div>
      </div>
      <template #footer>
        <button class="btn-outline" @click="closeModal">{{ $t('modal_cancel') }}</button>
        <button class="btn-primary" @click="saveMapping">{{ $t('modal_save') }}</button>
      </template>
    </BaseModal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseSlider from '../common/BaseSlider.vue';
import BaseSwitch from '../common/BaseSwitch.vue';
import BaseModal from '../common/BaseModal.vue';
import BaseSelect from '../common/BaseSelect.vue';
import type { AxisMapping } from '../../models/AxisMapping';

const store = useHardwareStore();
const analogValues = computed(() => store.status?.adc ?? Array(8).fill(0));

const activeIndices = computed(() => {
  return [0, 1, 2, 3, 4, 5].filter(i => {
    if (i < 3) return true;
    return store.gpio?.pin_mode[i] === 2; // Analog mode
  });
});

const getAxisLabel = (index: number) => {
  if (index < 3) return `AXIS ${['X', 'Y', 'Z'][index]}`;
  return `GPIO ${index}`;
};

const axisNames = ref<string[]>([]);
const mappings = ref<AxisMapping[]>([]);
const showModal = ref(false);
const editingIdx = ref<number | null>(null);

const DEFAULT_NAMES = ["Throttle", "Brake", "Clutch", "Aux 4", "Aux 5", "Aux 6", "Aux 7", "Aux 8"];

const initConfig = () => {
  const savedNames = localStorage.getItem('ffbeast_axis_names');
  axisNames.value = savedNames ? JSON.parse(savedNames) : [...DEFAULT_NAMES];

  const savedMappings = localStorage.getItem('ffbeast_axis_mappings');
  if (savedMappings) {
    mappings.value = JSON.parse(savedMappings);
  } else {
    mappings.value = Array(8).fill(0).map(() => ({
      name: '',
      min: 0,
      max: 4095,
      invert: false,
      keyLow: '',
      keyHigh: '',
      btnLow: null,
      btnHigh: null
    }));
  }
};

const getAxisName = (index: number) => axisNames.value[index] || DEFAULT_NAMES[index];

const updateMin = (idx: number, val: number) => {
  if (!store.adc) return;
  const mins = [...store.adc.raxis_min];
  mins[idx] = val;
  store.updateADC({ raxis_min: mins });
};

const updateMax = (idx: number, val: number) => {
  if (!store.adc) return;
  const maxes = [...store.adc.raxis_max];
  maxes[idx] = val;
  store.updateADC({ raxis_max: maxes });
};

const updateInvert = (idx: number, val: boolean) => {
  if (!store.adc) return;
  const invs = [...store.adc.raxis_invert];
  invs[idx] = val ? 1 : 0;
  console.log(`[InputsTab] Updating axis ${idx} invert to:`, val, '(value:', invs[idx], ')');
  store.updateADC({ raxis_invert: invs });
};

const buttonOptions = [
  { label: 'option_none', value: null },
  ...Array.from({ length: 32 }, (_, i) => ({ label: `Button ${i + 1}`, value: i }))
];

const KEY_LIST = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "Up", "Down", "Left", "Right", "Space", "Enter", "Tab", "Shift", "Ctrl", "Alt", "Esc"];
const keyOptions = [{ label: 'option_none', value: '' }, ...KEY_LIST.map(k => ({ label: k, value: k }))];

const editMapping = (index: number) => {
  editingIdx.value = index;
  showModal.value = true;
};

const closeModal = () => {
  showModal.value = false;
  editingIdx.value = null;
};

const saveMapping = () => {
  localStorage.setItem('ffbeast_axis_names', JSON.stringify(axisNames.value));
  localStorage.setItem('ffbeast_axis_mappings', JSON.stringify(mappings.value));
  closeModal();
};

const saveAxisName = () => {
  localStorage.setItem('ffbeast_axis_names', JSON.stringify(axisNames.value));
};

onMounted(initConfig);
</script>

<style scoped>
.inputs-tab {
  padding: var(--content-padding);
}

.axes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1rem;
}

.axis-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 1rem;
  backdrop-filter: blur(10px);
}

.axis-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.axis-info {
  display: flex;
  flex-direction: column;
}

.axis-label {
  font-size: 0.7rem;
  color: var(--text-dim);
  text-transform: uppercase;
}

.axis-name {
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--accent);
}

.axis-name-input {
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--accent);
  background: transparent;
  border: none;
  border-bottom: 1px solid transparent;
  outline: none;
  padding: 2px 4px;
  transition: all 0.2s;
  max-width: 200px;
}

.axis-name-input:hover {
  border-bottom-color: var(--border);
}

.axis-name-input:focus {
  border-bottom-color: var(--accent);
  background: rgba(255, 255, 255, 0.02);
}

.icon-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 0.85rem;
  opacity: 0.6;
  transition: opacity 0.2s;
  padding: 4px 8px;
  color: var(--text-dim);
}

.icon-btn:hover {
  opacity: 1;
  color: var(--accent);
}

.calibration-row {
  display: flex;
  gap: 2rem;
  align-items: center;
}

.monitor-mini {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  width: 40px;
}

.bar-bg {
  width: 12px;
  height: 120px;
  background: rgba(255,255,255,0.05);
  border-radius: 6px;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
}

.bar-fill {
  width: 100%;
  background: var(--accent);
  transition: height 0.05s linear;
}

.raw-val {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  color: var(--text-dim);
}

.config-grid {
  flex: 1;
  display: grid;
  grid-template-columns: 1fr;
  gap: 0.5rem;
}

.adc-config-section {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.adc-info {
  color: var(--text-dim);
  font-style: italic;
  font-size: 0.8rem;
  text-align: center;
  padding: 1rem;
}

.modal-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-group label {
  font-size: 0.85rem;
  color: var(--text-dim);
}

.base-input {
  background: rgba(0,0,0,0.3);
  border: 1px solid var(--border);
  color: var(--text-main);
  padding: 10px 14px;
  border-radius: var(--radius-sm);
  outline: none;
}

.base-input:focus {
  border-color: var(--accent);
}

.section-divider {
  font-size: 0.75rem;
  font-weight: 800;
  text-transform: uppercase;
  color: var(--accent);
  border-bottom: 1px solid var(--accent-muted);
  padding-bottom: 4px;
  margin-top: 0.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}
</style>
