<template>
  <div class="dual-threshold-slider">
    <div v-if="label" class="dual-threshold-slider__header">
      <label class="dual-threshold-slider__label">{{ label }}</label>
      <div class="dual-threshold-slider__values">
        <span class="dual-threshold-slider__value dual-threshold-slider__value--low">
          {{ t('general.low') }}: {{ lowValue }}
        </span>
        <span class="dual-threshold-slider__value dual-threshold-slider__value--high">
          {{ t('general.high') }}: {{ highValue }}
        </span>
      </div>
    </div>
    
    <div class="dual-threshold-slider__container">
      <div class="dual-threshold-slider__track">
        <!-- Hardware Axis Value Bar -->
        <div 
          v-if="rawValue !== undefined"
          class="dual-threshold-slider__axis-bar"
          :style="axisBarStyle"
        ></div>

        <!-- Active range visualization -->
        <div
          class="dual-threshold-slider__range"
          :style="rangeStyle"
        ></div>
        
        <!-- Low threshold slider -->
        <input
          type="range"
          :value="lowValue"
          :min="min"
          :max="inputLowMax"
          :step="step"
          :disabled="disabled"
          class="dual-threshold-slider__input dual-threshold-slider__input--low"
          @input="handleLowInput"
        />
        
        <!-- High threshold slider -->
        <input
          type="range"
          :value="highValue"
          :min="inputHighMin"
          :max="max"
          :step="step"
          :disabled="disabled"
          class="dual-threshold-slider__input dual-threshold-slider__input--high"
          @input="handleHighInput"
        />
      </div>
      
      <div class="dual-threshold-slider__ticks">
        <span class="dual-threshold-slider__tick">{{ min }}</span>
        <span class="dual-threshold-slider__tick">{{ max }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

interface Props {
  lowValue: number;
  highValue: number;
  min?: number;
  max?: number;
  step?: number;
  label?: string;
  disabled?: boolean;
  rawValue?: number;
  maxLow?: number;
  minHigh?: number;
}

const props = withDefaults(defineProps<Props>(), {
  min: 0,
  max: 32767,
  step: 1,
  label: '',
  disabled: false,
  rawValue: undefined,
  maxLow: undefined,
  minHigh: undefined,
});

const emit = defineEmits<{
  'update:lowValue': [value: number];
  'update:highValue': [value: number];
}>();

const { t } = useI18n();

const rangeStyle = computed(() => {
  const range = props.max - props.min;
  const lowPercent = ((props.lowValue - props.min) / range) * 100;
  const highPercent = ((props.highValue - props.min) / range) * 100;
  
  return {
    left: `${lowPercent}%`,
    width: `${highPercent - lowPercent}%`,
  };
});

const axisBarStyle = computed(() => {
  if (props.rawValue === undefined) return {};
  const range = props.max - props.min;
  const val = Math.max(props.min, Math.min(props.max, props.rawValue));
  const percent = ((val - props.min) / range) * 100;
  
  return {
    width: `${percent}%`
  };
});

// Computed strict limits for inputs
const inputLowMax = computed(() => {
  const collisionArg = props.highValue - props.step;
  if (props.maxLow !== undefined) {
    return Math.min(props.maxLow, collisionArg);
  }
  return collisionArg;
});

const inputHighMin = computed(() => {
  const collisionArg = props.lowValue + props.step;
  if (props.minHigh !== undefined) {
    return Math.max(props.minHigh, collisionArg);
  }
  return collisionArg;
});

function handleLowInput(event: Event) {
  const target = event.target as HTMLInputElement;
  let value = Number(target.value);
  
  // Enforce maxLow if present
  if (props.maxLow !== undefined && value > props.maxLow) {
    value = props.maxLow;
  }

  // Ensure low value doesn't exceed high value
  if (value < props.highValue) {
    emit('update:lowValue', value);
  }
}

function handleHighInput(event: Event) {
  const target = event.target as HTMLInputElement;
  let value = Number(target.value);
  
  // Enforce minHigh if present
  if (props.minHigh !== undefined && value < props.minHigh) {
    value = props.minHigh;
  }

  // Ensure high value doesn't go below low value
  if (value > props.lowValue) {
    emit('update:highValue', value);
  }
}
</script>

<style scoped>
.dual-threshold-slider {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}

.dual-threshold-slider__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
}

.dual-threshold-slider__label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-main);
}

.dual-threshold-slider__values {
  display: flex;
  gap: 16px;
}

.dual-threshold-slider__value {
  font-size: 12px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.dual-threshold-slider__value--low {
  color: var(--warning);
}

.dual-threshold-slider__value--high {
  color: var(--success);
}

.dual-threshold-slider__container {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 4px;
  width: 100%;
  box-sizing: border-box;
  padding: 0 10px;
}

.dual-threshold-slider__track {
  position: relative;
  height: 40px;
  width: 100%;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 4px;
}

.dual-threshold-slider__range {
  position: absolute;
  top: 50%;
  left: 0;
  transform: translateY(-50%);
  height: 6px;
  background: linear-gradient(
    90deg,
    var(--warning),
    var(--success)
  );
  border-radius: 3px;
  pointer-events: none;
  z-index: 1;
}

.dual-threshold-slider__input {
  position: absolute;
  left: 0;
  width: 100%;
  max-width: none;
  margin: 0;
  padding: 0;
  border: 0;
  height: 6px;
  top: 50%;
  transform: translateY(-50%);
  background: transparent;
  outline: none;
  -webkit-appearance: none;
  appearance: none;
  pointer-events: none;
  z-index: 2;
}


.dual-threshold-slider__input::-webkit-slider-track {
  background: transparent;
  height: 6px;
}

.dual-threshold-slider__input::-moz-range-track {
  background: transparent;
  height: 6px;
}

.dual-threshold-slider__input::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  cursor: pointer;
  pointer-events: auto;
  transition: all 0.2s ease;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
  border: 3px solid var(--bg-card);
  z-index: 2;
}

.dual-threshold-slider__input::-moz-range-thumb {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  cursor: pointer;
  pointer-events: auto;
  border: 3px solid var(--bg-card);
  transition: all 0.2s ease;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
  z-index: 2;
}

.dual-threshold-slider__input--low::-webkit-slider-thumb {
  background: var(--status-warning);
}

.dual-threshold-slider__input--low::-moz-range-thumb {
  background: var(--status-warning);
}

.dual-threshold-slider__input--high::-webkit-slider-thumb {
  background: var(--status-success);
}

.dual-threshold-slider__input--high::-moz-range-thumb {
  background: var(--status-success);
}

.dual-threshold-slider__input:hover::-webkit-slider-thumb {
  transform: scale(1.15);
}

.dual-threshold-slider__input:hover::-moz-range-thumb {
  transform: scale(1.15);
}

.dual-threshold-slider__input:active::-webkit-slider-thumb {
  transform: scale(1.2);
}

.dual-threshold-slider__input:active::-moz-range-thumb {
  transform: scale(1.2);
}

.dual-threshold-slider__ticks {
  display: flex;
  justify-content: space-between;
  padding: 0 2px;
}

.dual-threshold-slider__tick {
  font-size: 11px;
  color: var(--text-dim);
  font-family: 'JetBrains Mono', monospace;
}

.dual-threshold-slider__axis-bar {
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  height: 100%;
  background: rgba(255, 255, 255, 0.15); /* More visible */
  border-radius: 4px;
  pointer-events: none;
  z-index: 0;
  transition: width 0.05s linear;
}

.dual-threshold-slider__range {
  z-index: 1;
}

.dual-threshold-slider__input {
  z-index: 2;
}
</style>
