<template>
  <div class="base-slider">
    <div class="slider-header">
      <div class="label-group">
        <label v-if="label">{{ label }}</label>
        <div v-if="help" class="help-icon" :title="help">?</div>
      </div>
      <div class="input-group">
        <input 
          type="number" 
          :value="modelValue" 
          @input="updateValue"
          :min="min"
          :max="max"
          class="number-input"
        >
        <span class="suffix">{{ suffix }}</span>
      </div>
    </div>
    <div class="slider-container">
      <input 
        type="range" 
        :min="min" 
        :max="max" 
        :step="step" 
        :value="modelValue"
        @input="updateValue"
        class="range-input"
      >
      <div class="track-fill" :style="{ width: fillWidth + '%' }"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(defineProps<{
  modelValue: number;
  label?: string;
  help?: string;
  min?: number;
  max?: number;
  step?: number;
  suffix?: string;
}>(), {
  min: 0,
  max: 100,
  step: 1,
  suffix: ''
});

const emit = defineEmits(['update:modelValue']);

const fillWidth = computed(() => {
  const range = props.max - props.min;
  if (range === 0) return 0;
  return ((props.modelValue - props.min) / range) * 100;
});

const updateValue = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const value = target.type === 'number' ? parseFloat(target.value) : target.valueAsNumber;
  if (!isNaN(value)) {
    emit('update:modelValue', value);
  }
};
</script>

<style scoped>
.base-slider {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 1.2rem;
}

.slider-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.label-group {
  display: flex;
  align-items: center;
  gap: 6px;
}

label {
  font-size: 0.85rem;
  color: var(--text-dim);
}

.help-icon {
  width: 14px;
  height: 14px;
  background: var(--bg-sidebar);
  border: 1px solid var(--border-bright);
  border-radius: 50%;
  font-size: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-dim);
  cursor: help;
}

.input-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.number-input {
  width: 60px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border);
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: 0.85rem;
  padding: 2px 4px;
  border-radius: var(--radius-sm);
  text-align: right;
  outline: none;
}

.number-input:focus {
  border-color: var(--accent);
}

/* Remove arrows from number input */
.number-input::-webkit-outer-spin-button,
.number-input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.suffix {
  font-size: 0.75rem;
  color: var(--text-dim);
  width: 15px;
}

.slider-container {
  position: relative;
  height: 6px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 3px;
  display: flex;
  align-items: center;
}

.range-input {
  position: absolute;
  width: 100%;
  height: 100%;
  appearance: none;
  background: transparent;
  z-index: 2;
  cursor: pointer;
  margin: 0;
}

.range-input::-webkit-slider-thumb {
  appearance: none;
  width: 14px;
  height: 14px;
  background: var(--text-main);
  border: 2px solid var(--accent);
  border-radius: 50%;
  box-shadow: 0 0 10px var(--accent-glow);
  transition: transform 0.1s ease;
}

.range-input::-webkit-slider-thumb:hover {
  transform: scale(1.2);
}

.track-fill {
  position: absolute;
  height: 100%;
  background: var(--accent);
  border-radius: 3px;
  z-index: 1;
}
</style>
