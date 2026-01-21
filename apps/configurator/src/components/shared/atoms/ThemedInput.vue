<template>
  <input
    :type="type"
    :value="modelValue"
    :placeholder="placeholder"
    :disabled="disabled"
    :readonly="readonly"
    :min="min"
    :max="max"
    :step="step"
    class="themed-input"
    :class="{ 'themed-input--error': error, 'themed-input--disabled': disabled }"
    @input="handleInput"
    @blur="$emit('blur', $event)"
    @focus="$emit('focus', $event)"
  />
</template>

<script setup lang="ts">
interface Props {
  modelValue: string | number;
  type?: 'text' | 'number' | 'email' | 'password';
  placeholder?: string;
  disabled?: boolean;
  readonly?: boolean;
  error?: boolean;
  min?: number;
  max?: number;
  step?: number;
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  placeholder: '',
  disabled: false,
  readonly: false,
  error: false,
});

const emit = defineEmits<{
  'update:modelValue': [value: string | number];
  'blur': [event: FocusEvent];
  'focus': [event: FocusEvent];
}>();

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  const value = props.type === 'number' ? Number(target.value) : target.value;
  emit('update:modelValue', value);
}
</script>

<style scoped>
.themed-input {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-family: 'Outfit', sans-serif;
  font-size: 14px;
  transition: all 0.2s ease;
  outline: none;
}

.themed-input:hover:not(:disabled) {
  border-color: var(--accent-primary);
}

.themed-input:focus {
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(var(--accent-primary-rgb), 0.1);
}

.themed-input--error {
  border-color: var(--status-error);
}

.themed-input--error:focus {
  box-shadow: 0 0 0 3px rgba(var(--status-error-rgb), 0.1);
}

.themed-input--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.themed-input::placeholder {
  color: var(--text-tertiary);
}

/* Number input specific styles */
.themed-input[type="number"]::-webkit-inner-spin-button,
.themed-input[type="number"]::-webkit-outer-spin-button {
  opacity: 1;
}
</style>
