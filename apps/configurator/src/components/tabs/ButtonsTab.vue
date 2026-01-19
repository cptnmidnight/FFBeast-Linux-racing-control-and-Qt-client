<template>
  <div class="buttons-tab">
    <div class="grid-layout">
      <!-- Live Monitoring -->
      <ButtonsGrid :buttons="currentButtons" />

      <!-- Button Configuration -->
      <BaseCard :title="$t('group_buttons')">
        <div class="buttons-config-grid">
          <div v-for="(_, index) in buttonModes.length" :key="index" class="button-config-item">
            <span class="btn-idx-label">{{ $t('btn_label') }} {{ index + 1 }}</span>
            <BaseSelect 
              v-model="buttonModes[index]" 
              :options="modeOptions" 
              class="compact"
              :use-i18n="true"
              @update:model-value="save"
            />
          </div>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import BaseSelect from '../common/BaseSelect.vue';
import ButtonsGrid from '../monitor/ButtonsGrid.vue';

const store = useHardwareStore();
const currentButtons = computed(() => store.status?.buttons ?? 0);
const buttonModes = computed(() => store.gpio?.button_mode ?? []);

const modeOptions = [
  { label: 'btn_mode_none', value: 0 },
  { label: 'btn_mode_normal', value: 1 },
  { label: 'btn_mode_inverted', value: 2 },
  { label: 'btn_mode_pulse', value: 3 },
];

const save = async () => {
  if (store.gpio) {
    await store.updateGPIO({
      button_mode: buttonModes.value
    });
  }
};
</script>

<style scoped>
.buttons-tab {
  padding: var(--content-padding);
}

.grid-layout {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.buttons-config-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 1rem;
}

.button-config-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
  background: rgba(255, 255, 255, 0.02);
  padding: 10px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}

.btn-idx-label {
  font-size: 0.7rem;
  color: var(--text-dim);
  font-weight: 600;
}

.compact {
  margin-bottom: 0 !important;
}
</style>
