<template>
  <div class="pins-tab">
    <BaseCard :title="$t('group_pins')">
      <div class="pins-grid">
        <div v-for="(_, index) in 16" :key="index" class="pin-item">
          <div class="pin-header">
            <span class="pin-label">GPIO {{ index }}</span>
          </div>
          <BaseSelect 
            v-model="pinModes[index]" 
            :options="modeOptions" 
            class="compact"
            @update:model-value="save"
          />
        </div>
      </div>
    </BaseCard>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import BaseSelect from '../common/BaseSelect.vue';

const store = useHardwareStore();
const pinModes = reactive(store.gpio?.pin_mode ?? Array(16).fill(0));

const modeOptions = [
  { label: 'None', value: 0 },
  { label: 'Digital Input', value: 1 },
  { label: 'Analog Input', value: 2 },
  { label: 'SPI CS', value: 3 },
  { label: 'SPI SCK', value: 4 },
  { label: 'SPI MISO', value: 5 },
  { label: 'Enable Effects', value: 6 },
  { label: 'Center Reset', value: 7 },
  { label: 'Braking PWM', value: 8 },
  { label: 'Effect LED', value: 9 },
  { label: 'Reboot', value: 10 },
];

const save = () => {
  // store.updateGPIO({ pin_mode: pinModes });
};
</script>

<style scoped>
.pins-tab {
  padding: var(--content-padding);
}

.pins-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 1.5rem;
}

.pin-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
  background: rgba(255, 255, 255, 0.02);
  padding: 12px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border);
}

.pin-label {
  font-size: 0.8rem;
  color: var(--accent);
  font-family: var(--font-mono);
  font-weight: 800;
}

.compact {
  margin-bottom: 0 !important;
}
</style>
