<template>
  <div class="base-switch" @click.stop="toggle">
    <div :class="['toggle-track', { active: modelValue }]">
      <div class="toggle-thumb"></div>
    </div>
    <div v-if="label || help" class="label-container">
      <span v-if="label" class="label">{{ label }}</span>
      <div v-if="help" class="help-icon" :data-help="help">?</div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  modelValue: boolean;
  label?: string;
  help?: string;
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
  flex-shrink: 0;
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

.label-container {
  display: flex;
  align-items: center;
  gap: 8px;
}

.label {
  font-size: 0.9rem;
  color: var(--text-main);
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
</style>
