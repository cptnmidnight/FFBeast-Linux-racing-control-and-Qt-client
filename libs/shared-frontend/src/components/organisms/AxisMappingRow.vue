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
          help-key="help.axis_custom_name"
        />
      </div>
      <div class="axis-mapping">
        <div class="mapped-info" v-if="mappedKey || mappedButton">
          <span class="mapped-badge mapped-badge--key" v-if="mappedKey">{{ mappedKey }}</span>
          <span class="mapped-badge mapped-badge--btn" v-if="mappedButton">{{ mappedButton }}</span>
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
    </div>

    <!-- Narrow Axis Monitor -->
    <div class="axis-monitor-container">
      <AxisMonitor
        :value="liveValue"
        :min="0"
        :max="65535"
        :show-value="false"
        class="narrow-monitor"
      />
    </div>

    <!-- Calibration Controls (Only for Primary Axes 0-2) -->
    <div v-if="shouldShowCalibration" class="calibration-controls">
      <div class="control-grid">
        <ThemedSlider
          :model-value="min"
          :min="0"
          :max="65535"
          :label="$t('settings.min')"
          help-key="help.axis_min"
          @update:model-value="(v: number) => $emit('update:min', v)"
        />
        
        <ThemedSlider
          :model-value="max"
          :min="0"
          :max="65535"
          :label="$t('settings.max')"
          help-key="help.axis_max"
          @update:model-value="(v: number) => $emit('update:max', v)"
        />
      </div>

      <div class="control-grid">
        <ThemedSlider
          :model-value="smoothing"
          :min="0"
          :max="100"
          :label="$t('settings.axis.smoothing')"
          value-suffix="%"
          help-key="help.position_smoothing"
          @update:model-value="(v: number) => $emit('update:smoothing', v)"
        />

        <ThemedSwitch 
          :model-value="invert" 
          :label="$t('settings.axis.invert')" 
          help-key="help.axis_invert"
          @update:model-value="(v: boolean) => $emit('update:invert', v)"
        />
      </div>

      <div class="control-grid">
        <ThemedSlider
          :model-value="btnLow"
          :min="0"
          :max="100"
          :label="$t('settings.axis.buttons.low')"
          value-suffix="%"
          help-key="help.axis_buttons_low"
          @update:model-value="(v: number) => $emit('update:btnLow', v)"
        />
        
        <ThemedSlider
          :model-value="btnHigh"
          :min="0"
          :max="100"
          :label="$t('settings.axis.buttons.high')"
          value-suffix="%"
          help-key="help.axis_buttons_high"
          @update:model-value="(v: number) => $emit('update:btnHigh', v)"
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
import { useHardwareStream } from '../../composables/useHardwareStream';
import ThemedInput from '../atoms/ThemedInput.vue';
import ThemedSlider from '../atoms/ThemedSlider.vue';
import ThemedSwitch from '../atoms/ThemedSwitch.vue';
import AxisMonitor from '../molecules/AxisMonitor.vue';

interface Props {
  index: number;
  label: string;
  modelValue: string; // The Name
  rawValue: number;
  defaultName: string;
  min?: number;
  max?: number;
  invert?: boolean;
  smoothing?: number;
  btnLow?: number;
  btnHigh?: number;
  showCalibration?: boolean;
  mappedKey?: string;
  mappedButton?: string;
}

const props = withDefaults(defineProps<Props>(), {
  min: 0,
  max: 65535,
  invert: false,
  smoothing: 0,
  btnLow: 0,
  btnHigh: 100,
  showCalibration: undefined
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'save-name': [];
  'edit': [];
  'update:min': [value: number];
  'update:max': [value: number];
  'update:invert': [value: boolean];
  'update:smoothing': [value: number];
  'update:btnLow': [value: number];
  'update:btnHigh': [value: number];
}>();

const shouldShowCalibration = computed(() => {
  if (props.showCalibration !== undefined) return props.showCalibration;
  return props.index < 3;
});

const onNameChange = (val: string | number) => {
  emit('update:modelValue', String(val));
};

const { status: hardwareStream } = useHardwareStream();

const liveValue = computed(() => {
  if (hardwareStream.value?.adc) {
    const raw = hardwareStream.value.adc[props.index + 3] ?? 0;
    return Math.floor((raw * 65535) / 4095);
  }
  return props.rawValue + 3 || 0;
});
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
  font-size: 10px;
  text-transform: uppercase;
  color: var(--text-tertiary);
  font-weight: 800;
  letter-spacing: 0.05em;
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
  padding: 0;
  margin-top: -8px;
  margin-bottom: 4px;
}

:deep(.narrow-monitor .axis-monitor__track) {
  height: 6px !important;
  background: rgba(0, 0, 0, 0.3);
  border: none;
}

:deep(.narrow-monitor .axis-monitor__fill) {
  border-radius: 3px;
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

.axis-mapping {
  display: flex;
  align-items: center;
  gap: 12px;
}

.mapped-info {
  display: flex;
  gap: 4px;
}

.mapped-badge {
  font-size: 10px;
  font-weight: 700;
  padding: 2px 6px;
  border-radius: 4px;
  text-transform: uppercase;
  font-family: var(--font-mono);
}

.mapped-badge--key {
  background: rgba(var(--accent-primary-rgb), 0.2);
  color: var(--accent-primary);
  border: 1px solid rgba(var(--accent-primary-rgb), 0.3);
}

.mapped-badge--btn {
  background: rgba(var(--status-success-rgb, 0, 255, 136), 0.2);
  color: var(--status-success, #00ff88);
  border: 1px solid rgba(var(--status-success-rgb, 0, 255, 136), 0.3);
}
</style>
