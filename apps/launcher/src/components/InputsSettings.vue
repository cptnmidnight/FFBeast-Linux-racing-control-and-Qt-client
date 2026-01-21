<template>
  <div class="inputs-settings">
    <div class="axes-grid">
      <div v-for="(mapping, index) in localMappings" :key="index" class="axis-card">
        <div class="axis-header">
          <div class="axis-info">
            <span class="axis-label">{{ getAxisLabel(index) }}</span>
            <input 
              type="text" 
              v-model="mapping.name" 
              class="axis-name-input"
              :placeholder="DEFAULT_NAMES[index]"
              @blur="saveToLocal"
            />
          </div>
          <button class="icon-btn" @click="editMapping(index)">⚙️ {{ $t('btn_edit_mapping') }}</button>
        </div>
        
        <div class="adc-config-section">
            <div class="calibration-row">
                 <div class="monitor-mini">
                    <div class="bar-bg">
                        <div class="bar-fill" :style="{ height: getAxisPercentage(index) + '%' }"></div>
                    </div>
                    <span class="raw-val">{{ getAxisValue(index) }}</span>
                 </div>

                 <div class="config-grid">
                    <BaseSlider 
                        v-model.number="mapping.min" 
                        label="Min" 
                        :max="65535" 
                        @change="saveMapping"
                    />
                    <BaseSlider 
                        v-model.number="mapping.max" 
                        label="Max" 
                        :max="65535" 
                        @change="saveMapping"
                    />
                    <label class="invert-check">
                        <input type="checkbox" v-model="mapping.invert" @change="saveMapping"> 
                        {{ $t('settings.gamepad.inverted') }}
                    </label>
                 </div>
            </div>
            
            <div class="mapping-summary" v-if="mapping.keyLow || mapping.keyHigh">
                <small v-if="mapping.keyLow">Low: {{ mapping.keyLow }}</small>
                <small v-if="mapping.keyHigh">High: {{ mapping.keyHigh }}</small>
            </div>
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
          <input type="text" v-model="localMappings[editingIdx].name" class="base-input">
        </div>

        <div class="section-divider">{{ $t('axis_joystick_mapping') }}</div>
        <div class="form-row">
          <BaseSelect 
            v-model="localMappings[editingIdx].btnLow" 
            :options="buttonOptions" 
            :label="$t('axis_button_low')"
          />
          <BaseSelect 
            v-model="localMappings[editingIdx].btnHigh" 
            :options="buttonOptions" 
            :label="$t('axis_button_high')"
          />
        </div>

        <div class="section-divider">{{ $t('axis_keyboard_mapping') }}</div>
        <div class="form-row">
          <BaseSelect 
            v-model="localMappings[editingIdx].keyLow" 
            :options="keyOptions" 
            :label="$t('axis_start')"
          />
          <BaseSelect 
            v-model="localMappings[editingIdx].keyHigh" 
            :options="keyOptions" 
            :label="$t('axis_end')"
          />
        </div>
        
        <p class="hint">{{ $t('axis_mapping_hint') }}</p>
      </div>
      <template #footer>
        <button class="btn-outline" @click="closeModal">{{ $t('modal_cancel') }}</button>
        <button class="btn-primary" @click="saveMapping">{{ $t('modal_save') }}</button>
      </template>
    </BaseModal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useI18n } from 'vue-i18n';
import BaseSlider from './common/BaseSlider.vue';
import BaseModal from './common/BaseModal.vue';
import BaseSelect from './common/BaseSelect.vue';

const { t } = useI18n();

interface LocalMapping {
    name: string;
    min: number;
    max: number;
    invert: boolean;
    keyLow: string;
    keyHigh: string;
    btnLow: string; // Changed to string to match Select value type usually
    btnHigh: string;
}

// 6 Axes: Wheel, Accel, Brake, Clutch, Aux1, Aux2
const AXIS_COUNT = 6;
const DEFAULT_NAMES = computed(() => [
  t('axis_names.wheel'),
  t('axis_names.throttle'),
  t('axis_names.brake'),
  t('axis_names.clutch'),
  t('axis_names.aux') + ' 1',
  t('axis_names.aux') + ' 2'
]);

const localMappings = ref<LocalMapping[]>([]);
const showModal = ref(false);
const editingIdx = ref<number | null>(null);
const currentStatus = ref<any>(null);

const getAxisLabel = (index: number) => {
  if (index === 0) return t('axis_labels.x_wheel');
  return t('axis_labels.generic', { n: index + 1 });
};

const getAxisValue = (index: number) => {
    if (!currentStatus.value) return 0;
    if (index === 0) return currentStatus.value.position;
    // Map index 1..5 to adc[0]..adc[4]
    return currentStatus.value.adc[index - 1] || 0;
};

const getAxisPercentage = (index: number) => {
    const val = getAxisValue(index);
    const mapping = localMappings.value[index];
    if (!mapping) return 0;

    let min = mapping.min || 0;
    let max = mapping.max || 65535;
    
    // Normalize
    let range = max - min;
    let norm = range !== 0 ? (val - min) / range : 0;
    
    if (norm < 0) norm = 0;
    if (norm > 1) norm = 1;
    
    if (mapping.invert) norm = 1 - norm;
    
    return norm * 100;
};

const initConfig = async () => {
  // Try to load from backend first
  try {
      const backendMappings: any[] = await invoke('get_keyboard_mapping');
      
      // Initialize with defaults
      const initial: LocalMapping[] = Array(AXIS_COUNT).fill(0).map((_, i) => ({
        name: DEFAULT_NAMES.value[i],
        min: 0,
        max: 65535,
        invert: false,
        keyLow: '',
        keyHigh: '',
        btnLow: '',
        btnHigh: ''
      }));

      // Merge backend data
      if (backendMappings && backendMappings.length > 0) {
          backendMappings.forEach(bm => {
              if (bm.index < AXIS_COUNT) {
                  initial[bm.index].name = bm.name;
                  initial[bm.index].keyLow = bm.key_low;
                  initial[bm.index].keyHigh = bm.key_high;
                  initial[bm.index].btnLow = bm.btn_low;
                  initial[bm.index].btnHigh = bm.btn_high;
                  initial[bm.index].min = bm.min !== undefined ? bm.min : 0;
                  initial[bm.index].max = bm.max !== undefined ? bm.max : 65535;
                  initial[bm.index].invert = bm.inverted !== undefined ? bm.inverted : false;
              }
          });
      }
      
      localMappings.value = initial;

  } catch (e) {
      console.error("Failed to load mappings", e);
      // Fallback
       localMappings.value = Array(AXIS_COUNT).fill(0).map((_, i) => ({
        name: DEFAULT_NAMES.value[i],
        min: 0,
        max: 65535,
        invert: false,
        keyLow: '',
        keyHigh: '',
        btnLow: '',
        btnHigh: ''
      }));
  }
};

const saveToLocal = () => {
    // Just placeholder if we want to save names locally
};

const saveMapping = async () => {
    // Save to backend
    // Convert LocalMapping to Backend KeyMapping
    if (localMappings.value.length === 0) return;

    const backendPayload = localMappings.value.map((m, i) => ({
        index: i,
        name: m.name,
        key_low: m.keyLow,
        key_high: m.keyHigh,
        btn_low: m.btnLow || "",
        btn_high: m.btnHigh || "",
        min: m.min || 0,
        max: m.max || 65535,
        inverted: m.invert || false
    }));

    try {
        await invoke('set_keyboard_mapping', { mappings: backendPayload });
        // Also ensure service is started/restarted?
        // Ideally backend handles hot reload of mappings
    } catch (e) {
        console.error("Failed to save mappings", e);
    } // Removed alert for auto-save silently
    
    // Only close modal if it's open
    if (showModal.value) closeModal();
};

const editMapping = (index: number) => {
  editingIdx.value = index;
  showModal.value = true;
};

const closeModal = () => {
  showModal.value = false;
  editingIdx.value = null;
};

// Listen to hardware
let unlisten: any;

onMounted(async () => {
    await initConfig();
    unlisten = await listen('wheel-status', (event: any) => {
        currentStatus.value = event.payload;
    });
});

onUnmounted(() => {
    if (unlisten) unlisten();
});

// Options
const buttonOptions = [
  { label: 'Nenhum', value: '' },
  ...Array.from({ length: 32 }, (_, i) => ({ label: `Botão ${i + 1}`, value: `BUTTON_${i}` }))
];

const KEY_LIST = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "Up", "Down", "Left", "Right", "Space", "Enter", "Tab", "Shift", "Ctrl", "Alt", "Esc"];
const keyOptions = [{ label: 'Nenhuma', value: '' }, ...KEY_LIST.map(k => ({ label: k, value: k }))];

</script>

<style scoped>
.inputs-settings {
    padding-top: 1rem;
}

.axes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}

.axis-card {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1rem;
  background: rgba(255,255,255,0.02);
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
  font-size: 0.75rem;
  color: var(--text-dim, #888);
  text-transform: uppercase;
  font-weight: 700;
}

.axis-name-input {
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--primary-color);
  background: transparent;
  border: none;
  border-bottom: 1px solid transparent;
  outline: none;
  padding: 2px 0;
  transition: all 0.2s;
  max-width: 180px;
}

.axis-name-input:focus {
  border-bottom-color: var(--primary-color);
}

.icon-btn {
  background: rgba(255,255,255,0.1);
  border: none;
  cursor: pointer;
  font-size: 0.85rem;
  border-radius: 4px;
  padding: 6px 12px;
  color: var(--text-color);
  transition: all 0.2s;
}

.icon-btn:hover {
  background: var(--primary-color);
  color: white;
}

.calibration-row {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.monitor-mini {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  width: 50px;
}

.bar-bg {
  width: 14px;
  height: 100px;
  background: rgba(255,255,255,0.1);
  border-radius: 7px;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
}

.bar-fill {
  width: 100%;
  background: var(--primary-color);
  transition: height 0.05s linear;
  min-height: 2px;
}

.raw-val {
  font-family: monospace;
  font-size: 0.7rem;
  color: #888;
}

.config-grid {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.modal-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.section-divider {
  font-size: 0.8rem;
  font-weight: 800;
  text-transform: uppercase;
  color: var(--primary-color);
  border-bottom: 1px solid rgba(255,255,255,0.1);
  padding-bottom: 4px;
  margin-top: 0.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.hint {
    font-size: 0.8rem;
    color: #888;
    font-style: italic;
    margin-top: -1rem;
}

.btn-primary, .btn-outline {
    padding: 10px 20px;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
}

.btn-primary {
    background: var(--primary-color);
    color: white;
    border: none;
}

.btn-outline {
    background: transparent;
    border: 1px solid #666;
    color: var(--text-color);
}

.mapping-summary {
    display: flex;
    gap: 10px;
    margin-top: 10px;
    opacity: 0.7;
}

.base-input {
  background: rgba(0,0,0,0.3);
  border: 1px solid var(--border-color);
  color: var(--text-color);
  padding: 10px 14px;
  border-radius: 8px;
  outline: none;
}

.invert-check {
  font-size: 0.8rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-top: 0.5rem;
  color: var(--text-color);
  cursor: pointer;
}

.invert-check input {
  accent-color: var(--primary-color);
}
</style>
