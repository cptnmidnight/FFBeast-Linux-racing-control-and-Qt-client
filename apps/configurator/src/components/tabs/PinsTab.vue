<template>
  <div class="pins-tab">
    <BaseCard :title="$t('group_pins')">
      <div class="pins-list">
        <div v-for="(_, index) in pinCount" :key="index" class="pin-row">
          <span class="pin-label">{{ $t('pin_label') }} {{ index }}</span>
          <BaseSelect 
            v-model="pinModes[index]" 
            :options="modeOptions" 
            class="compact"
            :use-i18n="true"
            @update:model-value="save"
          />
        </div>
      </div>
    </BaseCard>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import BaseSelect from '../common/BaseSelect.vue';

const store = useHardwareStore();
const pinModes = computed(() => store.gpio?.pin_mode ?? []);
const pinCount = computed(() => pinModes.value.length);

const modeOptions = [
  { label: 'pin_mode_none', value: 0 },
  { label: 'pin_mode_gpio', value: 1 },
  { label: 'pin_mode_analog', value: 2 },
  { label: 'pin_mode_spi_cs', value: 3 },
  { label: 'pin_mode_spi_sck', value: 4 },
  { label: 'pin_mode_spi_miso', value: 5 },
  { label: 'pin_mode_enable_effects', value: 6 },
  { label: 'pin_mode_center_reset', value: 7 },
  { label: 'pin_mode_braking_pwm', value: 8 },
  { label: 'pin_mode_effect_led', value: 9 },
  { label: 'pin_mode_reboot', value: 10 },
];

const save = async () => {
  if (store.gpio) {
    await store.updateGPIO({
      pin_mode: pinModes.value
    });
  }
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
