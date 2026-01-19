<template>
  <div class="base-select">
    <label v-if="label">{{ label }}</label>
    <div class="select-wrapper">
      <select :value="modelValue" @change="updateValue" class="select-input">
        <option v-for="opt in options" :key="opt.value" :value="opt.value">
          {{ useI18n ? $t(opt.label) : opt.label }}
        </option>
      </select>
      <div class="select-arrow">▼</div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Option {
  label: string;
  value: any;
}

defineProps<{
  modelValue: any;
  options: Option[];
  label?: string;
  useI18n?: boolean;
}>();

const emit = defineEmits(['update:modelValue']);

const updateValue = (event: Event) => {
  const target = event.target as HTMLSelectElement;
  let val: any = target.value;
  // If numeric, cast it
  if (!isNaN(Number(val)) && val !== '') {
    val = Number(val);
  }
  emit('update:modelValue', val);
};
</script>

<style scoped>
.base-select {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 1.25rem;
}

label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.select-wrapper {
  position: relative;
  width: 100%;
}

.select-input {
  width: 100%;
  appearance: none;
  background: rgba(15, 15, 15, 0.4);
  border: 1px solid var(--border);
  color: var(--text-main);
  padding: 10px 14px;
  border-radius: var(--radius-sm);
  font-size: 0.85rem;
  outline: none;
  cursor: pointer;
  transition: all 0.2s ease;
  backdrop-filter: blur(5px);
}

.select-input:hover {
  background: rgba(255, 255, 255, 0.05);
  border-color: var(--border-bright);
}

.select-input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 10px var(--accent-glow);
  background: rgba(0, 0, 0, 0.6);
}

.select-arrow {
  position: absolute;
  right: 12px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 0.6rem;
  color: var(--text-dim);
  pointer-events: none;
}

option {
  background: #111;
  color: #fff;
  padding: 10px;
}
</style>
