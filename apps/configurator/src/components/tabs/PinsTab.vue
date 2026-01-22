<template>
  <div class="pins-tab">
    <div class="pins-grid">
      <div 
        v-for="index in pinCount" 
        :key="index" 
        class="pin-card"
      >
        <span class="pin-label">{{ $t('labels.pin') }} {{ index - 1 }}</span>
        <BaseSelect 
          :model-value="pinModes[index - 1]" 
          :options="modeOptions" 
          class="compact"
          :use-i18n="true"
          :disabled="!isConnected"
          @update:model-value="v => updatePinMode(index - 1, v)"
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
const pinModes = computed(() => store.gpio?.pin_mode ?? new Array(10).fill(0));
const pinCount = computed(() => pinModes.value.length);
const isConnected = computed(() => store.isConnected);

const modeOptions = [
  { label: 'pins.modes.none', value: 0 },
  { label: 'pins.modes.gpio', value: 1 },
  { label: 'pins.modes.analog', value: 2 },
  { label: 'pins.modes.spi_cs', value: 3 },
  { label: 'pins.modes.spi_sck', value: 4 },
  { label: 'pins.modes.spi_miso', value: 5 },
  { label: 'pins.modes.enable_effects', value: 6 },
  { label: 'pins.modes.center_reset', value: 7 },
  { label: 'pins.modes.braking_pwm', value: 8 },
  { label: 'pins.modes.effect_led', value: 9 },
  { label: 'pins.modes.reboot', value: 10 },
];

const updatePinMode = async (index: number, value: number) => {
  if (!store.isConnected) return;
  
  console.log(`[PinsTab] Pin ${index} mode changed to:`, value, `(${modeOptions.find(m => m.value === value)?.label})`);
  
  if (store.gpio) {
    const newModes = [...pinModes.value];
    newModes[index] = value;
    
    await store.updateGPIO({
      pin_mode: newModes
    });
    
    console.log(`[PinsTab] Updated pin_mode array:`, newModes);
  }
};
</script>

<style scoped>
.pins-tab {
  padding: var(--content-padding);
}

.pins-grid {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.pin-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  padding: 0.75rem;
  backdrop-filter: blur(10px);
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  transition: all 0.2s;
}

.pin-card:hover {
  border-color: var(--accent);
  box-shadow: 0 0 0 1px var(--accent-muted);
}

.pin-label {
  font-size: 0.7rem;
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
  font-size: 0.8rem;
  padding: 6px 10px;
  transition: all 0.2s;
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
