<template>
  <div class="themed-select">
    <label v-if="label" class="themed-select__label">{{ label }}</label>
    <select
      :value="modelValue"
      :disabled="disabled"
      class="themed-select__input"
      :class="{ 'themed-select__input--disabled': disabled }"
      @change="handleChange"
    >
      <option v-if="placeholder" value="" disabled>{{ placeholder }}</option>
      <option
        v-for="option in options"
        :key="getOptionValue(option)"
        :value="getOptionValue(option)"
      >
        {{ getOptionLabel(option) }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
export interface SelectOption {
  value: string | number;
  label: string;
}

interface Props {
  modelValue: string | number;
  options: SelectOption[] | string[] | number[];
  label?: string;
  placeholder?: string;
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  label: '',
  placeholder: '',
  disabled: false,
});

const emit = defineEmits<{
  'update:modelValue': [value: string | number];
  'change': [value: string | number];
}>();

function getOptionValue(option: SelectOption | string | number): string | number {
  if (typeof option === 'object' && option !== null) {
    return option.value;
  }
  return option;
}

function getOptionLabel(option: SelectOption | string | number): string {
  if (typeof option === 'object' && option !== null) {
    return option.label;
  }
  return String(option);
}

function handleChange(event: Event) {
  const target = event.target as HTMLSelectElement;
  const value = target.value;
  
  // Try to preserve the original type
  const numValue = Number(value);
  const finalValue = isNaN(numValue) ? value : numValue;
  
  emit('update:modelValue', finalValue);
  emit('change', finalValue);
}
</script>

<style scoped>
.themed-select {
  display: flex;
  flex-direction: column;
  gap: 6px;
  width: 100%;
}

.themed-select__label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-main);
}

.themed-select__input {
  width: 100%;
  padding: 8px 32px 8px 12px;
  background: var(--bg-sidebar);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-main);
  font-family: 'Outfit', sans-serif;
  font-size: 14px;
  cursor: pointer;
  outline: none;
  transition: all 0.2s ease;
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23888' d='M6 9L1 4h10z'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
}

.themed-select__input:hover:not(:disabled) {
  border-color: var(--accent);
}

.themed-select__input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px rgba(0, 212, 255, 0.1);
}

.themed-select__input--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.themed-select__input option {
  background: var(--bg-sidebar);
  color: var(--text-main);
  padding: 8px;
}
</style>
