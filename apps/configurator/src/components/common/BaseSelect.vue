<template>
  <div class="base-select">
    <label v-if="label">{{ label }}</label>
    <select :value="modelValue" @change="updateValue" class="select-input">
      <option v-for="opt in options" :key="opt.value" :value="opt.value">
        {{ opt.label }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
interface Option {
  label: string;
  value: any;
}

const props = defineProps<{
  modelValue: any;
  options: Option[];
  label?: string;
}>();

const emit = defineEmits(['update:modelValue']);

const updateValue = (event: Event) => {
  const target = event.target as HTMLSelectElement;
  emit('update:modelValue', target.value);
};
</script>

<style scoped>
.base-select {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 1.5rem;
}

label {
  font-size: 0.85rem;
  color: var(--text-dim);
}

.select-input {
  background: rgba(0, 0, 0, 0.2);
  border: 1px solid var(--border);
  color: var(--text-main);
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  font-size: 0.9rem;
  outline: none;
  cursor: pointer;
}

.select-input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 5px var(--accent-glow);
}
</style>
