<template>
  <div class="themed-slider">
    <div v-if="showLabel" class="themed-slider__header">
      <label v-if="label" class="themed-slider__label">{{ label }}</label>
      <span v-if="showValue" class="themed-slider__value">{{ displayValue }}</span>
    </div>
    <input
      type="range"
      :value="modelValue"
      :min="min"
      :max="max"
      :step="step"
      :disabled="disabled"
      class="themed-slider__input"
      :class="{ 'themed-slider__input--disabled': disabled }"
      @input="handleInput"
      @change="$emit('change', Number(($event.target as HTMLInputElement).value))"
    />
    <div v-if="showTicks" class="themed-slider__ticks">
      <span class="themed-slider__tick">{{ min }}</span>
      <span v-if="showMiddleTick" class="themed-slider__tick">{{ middleValue }}</span>
      <span class="themed-slider__tick">{{ max }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  modelValue: number;
  min?: number;
  max?: number;
  step?: number;
  label?: string;
  disabled?: boolean;
  showValue?: boolean;
  showLabel?: boolean;
  showTicks?: boolean;
  showMiddleTick?: boolean;
  valueFormatter?: (value: number) => string;
}

const props = withDefaults(defineProps<Props>(), {
  min: 0,
  max: 100,
  step: 1,
  label: '',
  disabled: false,
  showValue: true,
  showLabel: true,
  showTicks: false,
  showMiddleTick: false,
});

const emit = defineEmits<{
  'update:modelValue': [value: number];
  'change': [value: number];
}>();

const displayValue = computed(() => {
  if (props.valueFormatter) {
    return props.valueFormatter(props.modelValue);
  }
  return props.modelValue.toString();
});

const middleValue = computed(() => {
  return Math.round((props.min + props.max) / 2);
});

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  emit('update:modelValue', Number(target.value));
}
</script>

<style scoped>
.themed-slider {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
}

.themed-slider__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.themed-slider__label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.themed-slider__value {
  font-size: 14px;
  font-weight: 600;
  color: var(--accent-primary);
  font-family: 'JetBrains Mono', monospace;
}

.themed-slider__input {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--bg-secondary);
  outline: none;
  -webkit-appearance: none;
  appearance: none;
  cursor: pointer;
  transition: all 0.2s ease;
}

.themed-slider__input::-webkit-slider-track {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--bg-secondary);
}

.themed-slider__input::-moz-range-track {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--bg-secondary);
}

.themed-slider__input::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--accent-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.themed-slider__input::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--accent-primary);
  cursor: pointer;
  border: none;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.themed-slider__input:hover::-webkit-slider-thumb {
  transform: scale(1.1);
  box-shadow: 0 0 0 6px rgba(var(--accent-primary-rgb), 0.1);
}

.themed-slider__input:hover::-moz-range-thumb {
  transform: scale(1.1);
  box-shadow: 0 0 0 6px rgba(var(--accent-primary-rgb), 0.1);
}

.themed-slider__input:active::-webkit-slider-thumb {
  transform: scale(1.15);
}

.themed-slider__input:active::-moz-range-thumb {
  transform: scale(1.15);
}

.themed-slider__input--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.themed-slider__input--disabled::-webkit-slider-thumb {
  cursor: not-allowed;
}

.themed-slider__input--disabled::-moz-range-thumb {
  cursor: not-allowed;
}

.themed-slider__ticks {
  display: flex;
  justify-content: space-between;
  padding: 0 2px;
}

.themed-slider__tick {
  font-size: 11px;
  color: var(--text-tertiary);
  font-family: 'JetBrains Mono', monospace;
}
</style>
