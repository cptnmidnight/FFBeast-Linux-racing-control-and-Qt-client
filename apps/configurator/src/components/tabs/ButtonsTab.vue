<template>
  <div class="buttons-tab">
    <div class="buttons-grid">
      <div 
        v-for="index in 32" 
        :key="index" 
        class="button-card"
      >
        <span class="btn-label">{{ $t('buttons.label') }} {{ index }}</span>
        <BaseSelect 
          :model-value="buttonModes[index - 1]" 
          :options="modeOptions" 
          class="compact"
          :use-i18n="true"
          :disabled="!isConnected"
          @update:model-value="v => updateButtonMode(index - 1, v)"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseSelect from '../common/BaseSelect.vue';

const store = useHardwareStore();
const buttonModes = computed(() => store.gpio?.button_mode ?? new Array(32).fill(0));
const isConnected = computed(() => store.isConnected);

const modeOptions = [
  { label: 'modes.btn.none', value: 0 },
  { label: 'modes.btn.normal', value: 1 },
  { label: 'modes.btn.inverted', value: 2 },
  { label: 'modes.btn.pulse', value: 3 },
];

const updateButtonMode = async (index: number, value: number) => {
  if (!store.isConnected) return;

  console.log(`[ButtonsTab] Button ${index + 1} mode changed to:`, value, `(${modeOptions.find(m => m.value === value)?.label})`);
  
  if (store.gpio) {
    const newModes = [...buttonModes.value];
    newModes[index] = value;
    
    await store.updateGPIO({
      button_mode: newModes
    });
    
    console.log(`[ButtonsTab] Updated button_mode array:`, newModes);
  }
};
</script>

<style scoped>
.buttons-tab {
  padding: var(--content-padding);
}

.buttons-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 0.75rem;
}

.button-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  padding: 0.75rem;
  backdrop-filter: blur(10px);
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  align-items: center;
  transition: all 0.2s;
}

.button-card:hover {
  border-color: var(--accent);
  box-shadow: 0 0 0 1px var(--accent-muted);
}

.btn-label {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  line-height: 1;
}

.compact {
  margin-bottom: 0 !important;
  width: 100%;
}

.compact :deep(select) {
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-main);
  width: 100%;
  font-size: 0.75rem;
  padding: 6px 8px;
  transition: all 0.2s;
  text-align: center;
}

.compact :deep(select):hover {
  border-color: var(--accent);
  background: rgba(0, 0, 0, 0.5);
}

.compact :deep(select):focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-muted);
  outline: none;
}
</style>
