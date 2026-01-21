<template>
  <div class="axis-mapping-row">
    <div class="axis-header">
      <div class="axis-info">
        <span class="axis-label">{{ label }}</span>
        <ThemedInput
          :model-value="modelValue"
          @update:model-value="onNameChange"
          @blur="$emit('save-name')"
          class="axis-name-input"
          :placeholder="defaultName"
        />
      </div>
      <button 
        class="edit-btn" 
        @click="$emit('edit')"
        :title="$t('help.edit_mapping')"
      >
        <span>⚙️</span>
        <span class="btn-text">{{ $t('buttons.edit_mapping') }}</span>
      </button>
    </div>

    <!-- Live Monitor -->
    <div class="axis-monitor-container">
      <AxisMonitor
        :value="rawValue"
        :min="0"
        :max="32767"
        orientation="horizontal"
        :show-value="true"
        :normalized="false"
      />
    </div>

    <!-- Calibration Controls (Only for Primary Axes 0-2) -->
    <div v-if="index < 3" class="calibration-controls">
      <ThemedSlider
        :model-value="store.adc?.raxis_min[index] ?? 0"
        :min="0"
        :max="32767"
        :label="$t('settings.min')"
        @update:model-value="v => updateMin(v)"
      />
      
      <ThemedSlider
        :model-value="store.adc?.raxis_max[index] ?? 32767"
        :min="0"
        :max="32767"
        :label="$t('settings.max')"
        @update:model-value="v => updateMax(v)"
      />

      <div class="control-grid">
        <ThemedSlider
          :model-value="store.adc?.raxis_smoothing[index] ?? 0"
          :min="0"
          :max="100"
          :label="$t('settings.axis.smoothing') || 'Smoothing'"
          :value-formatter="v => `${v}%`"
          @update:model-value="v => updateSmoothing(v)"
        />

        <!-- Invert Switch (using simple checkbox for now or button) -->
        <div class="invert-control">
           <label>{{ $t('settings.axis.invert') }}</label>
           <div 
             class="toggle-switch"
             :class="{ active: isInverted }"
             @click="toggleInvert"
           >
             <div class="toggle-track"></div>
             <div class="toggle-thumb"></div>
           </div>
        </div>
      </div>

      <div class="control-grid">
        <ThemedSlider
          :model-value="store.adc?.raxis_to_button_low[index] ?? 0"
          :min="0"
          :max="100"
          :label="$t('settings.axis.buttons.low') || 'Button Low'"
          :value-formatter="v => `${v}%`"
          @update:model-value="v => updateBtnLow(v)"
        />
        
        <ThemedSlider
          :model-value="store.adc?.raxis_to_button_high[index] ?? 100"
          :min="0"
          :max="100"
          :label="$t('settings.axis.buttons.high') || 'Button High'"
          :value-formatter="v => `${v}%`"
          @update:model-value="v => updateBtnHigh(v)"
        />
      </div>
    </div>
    
    <div v-else class="adc-info">
      <p>{{ $t('settings.min_help') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import ThemedInput from '../shared/atoms/ThemedInput.vue';
import ThemedSlider from '../shared/atoms/ThemedSlider.vue';
import AxisMonitor from '../shared/molecules/AxisMonitor.vue';

interface Props {
  index: number;
  label: string;
  modelValue: string; // The Name
  rawValue: number;
  defaultName: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  'update:modelValue': [value: string];
  'save-name': [];
  'edit': [];
}>();

const store = useHardwareStore();

const isInverted = computed(() => {
  return (store.adc?.raxis_invert[props.index] ?? 0) === 1;
});

const onNameChange = (val: string | number) => {
  emit('update:modelValue', String(val));
};

// Store Updates
const updateMin = (val: number) => {
  if (!store.adc) return;
  const mins = [...store.adc.raxis_min];
  mins[props.index] = val;
  store.updateADC({ raxis_min: mins });
};

const updateMax = (val: number) => {
  if (!store.adc) return;
  const maxes = [...store.adc.raxis_max];
  maxes[props.index] = val;
  store.updateADC({ raxis_max: maxes });
};

const toggleInvert = () => {
  if (!store.adc) return;
  const invs = [...store.adc.raxis_invert];
  invs[props.index] = invs[props.index] === 1 ? 0 : 1;
  store.updateADC({ raxis_invert: invs });
};

const updateSmoothing = (val: number) => {
  if (!store.adc) return;
  const vals = [...store.adc.raxis_smoothing];
  vals[props.index] = val;
  store.updateADC({ raxis_smoothing: vals });
};

const updateBtnLow = (val: number) => {
  if (!store.adc) return;
  const vals = [...store.adc.raxis_to_button_low];
  vals[props.index] = val;
  store.updateADC({ raxis_to_button_low: vals });
};

const updateBtnHigh = (val: number) => {
  if (!store.adc) return;
  const vals = [...store.adc.raxis_to_button_high];
  vals[props.index] = val;
  store.updateADC({ raxis_to_button_high: vals });
};
</script>

<style scoped>
.axis-mapping-row {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  transition: border-color 0.2s;
}

.axis-mapping-row:hover {
  border-color: var(--accent-primary);
}

.axis-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.axis-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.axis-label {
  font-size: 11px;
  text-transform: uppercase;
  color: var(--text-tertiary);
  font-weight: 700;
  letter-spacing: 0.5px;
}

.axis-name-input {
  max-width: 200px;
}

/* Customizing ThemedInput for header look */
:deep(.axis-name-input input) {
  background: transparent;
  border: none;
  border-bottom: 1px solid transparent;
  border-radius: 0;
  padding: 0 0 4px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--accent-primary);
}

:deep(.axis-name-input input:focus) {
  box-shadow: none;
  border-bottom-color: var(--accent-primary);
}

:deep(.axis-name-input input:hover:not(:disabled)) {
  border-bottom-color: var(--border-color);
}

.edit-btn {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  padding: 6px 12px;
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  transition: all 0.2s;
}

.edit-btn:hover {
  background: rgba(var(--accent-primary-rgb), 0.1);
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.axis-monitor-container {
  padding: 4px 0;
}

.calibration-controls {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--border-color);
}

.control-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  align-items: center;
}

.adc-info {
  text-align: center;
  font-size: 12px;
  color: var(--text-tertiary);
  font-style: italic;
  padding: 8px;
  border-top: 1px solid var(--border-color);
}

/* Toggle Switch Styling */
.invert-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.invert-control label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.toggle-switch {
  width: 44px;
  height: 24px;
  background: var(--bg-secondary);
  border-radius: 12px;
  position: relative;
  cursor: pointer;
  transition: background 0.2s;
  border: 1px solid var(--border-color);
}

.toggle-switch.active {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
}

.toggle-thumb {
  width: 18px;
  height: 18px;
  background: #fff;
  border-radius: 50%;
  position: absolute;
  top: 2px;
  left: 2px;
  transition: transform 0.2s cubic-bezier(0.4, 0.0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0,0,0,0.3);
}

.toggle-switch.active .toggle-thumb {
  transform: translateX(20px);
}
</style>
