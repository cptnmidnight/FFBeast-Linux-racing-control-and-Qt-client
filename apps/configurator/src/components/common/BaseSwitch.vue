<template>
  <div class="base-switch" @click="toggle">
    <div :class="['toggle-track', { active: modelValue }]">
      <div class="toggle-thumb"></div>
    </div>
    <span v-if="label" class="label">{{ label }}</span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  modelValue: boolean;
  label?: string;
}>();

const emit = defineEmits(['update:modelValue']);

const toggle = () => {
  emit('update:modelValue', !props.modelValue);
};
</script>

<style scoped>
.base-switch {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  padding: 8px 0;
  user-select: none;
}

.toggle-track {
  width: 40px;
  height: 20px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  position: relative;
  transition: background 0.2s ease;
  border: 1px solid var(--border);
}

.toggle-track.active {
  background: var(--accent-muted);
  border-color: var(--accent);
}

.toggle-thumb {
  width: 14px;
  height: 14px;
  background: var(--text-dim);
  border-radius: 50%;
  position: absolute;
  top: 2px;
  left: 3px;
  transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.toggle-track.active .toggle-thumb {
  left: 21px;
  background: var(--accent);
  box-shadow: 0 0 8px var(--accent-glow);
}

.label {
  font-size: 0.9rem;
  color: var(--text-main);
}
</style>
