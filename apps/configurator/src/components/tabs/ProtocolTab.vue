<template>
  <div class="protocol-tab">
    <div class="card wide">
      <h3>{{ $t('group_extension') }}</h3>
      <div class="setting-row">
        <span class="setting-label">
          {{ $t('setting_ext_mode') }}
        </span>
        <BaseSelect 
          v-model="extensionMode" 
          :options="modeOptions" 
          :use-i18n="true"
          class="protocol-select"
          @update:model-value="handleModeChange"
        />
      </div>
    </div>
    
    <div class="card wide protocol-info-box">
      <h3 class="info-title">📘 {{ $t(currentModeTitle) }}</h3>
      
      <div class="protocol-info-section">
        <h4>{{ $t('protocol_desc_title') }}</h4>
        <p>{{ $t(`protocol_desc_${currentModeDetail}`) }}</p>
      </div>
      
      <div class="protocol-info-section">
        <h4>{{ $t('protocol_compat_title') }}</h4>
        <p>{{ $t(`protocol_compat_${currentModeDetail}`) }}</p>
      </div>
      
      <div class="protocol-info-section">
        <h4>{{ $t('protocol_config_title') }}</h4>
        <p>{{ $t(`protocol_config_${currentModeDetail}`) }}</p>
      </div>
      
      <div class="protocol-info-section">
        <h4>💡 {{ $t('protocol_tips_title') }}</h4>
        <p>{{ $t(`protocol_tips_${currentModeDetail}`) }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseSelect from '../common/BaseSelect.vue';

const store = useHardwareStore();
const extensionMode = ref(store.gpio?.extension_mode ?? 0);

const modeOptions = [
  { label: 'mode_none', value: 0 },
  { label: 'mode_buttons', value: 1 },
  { label: 'mode_spi_tm', value: 2 },
  { label: 'mode_spi_fanatec', value: 3 },
];

const titleKeys = ['mode_none', 'mode_buttons', 'mode_spi_tm', 'mode_spi_fanatec'];
const detailKeys = ['none', 'buttons', 'tm', 'fanatec'];

const currentModeTitle = computed(() => titleKeys[extensionMode.value] || 'mode_none');
const currentModeDetail = computed(() => detailKeys[extensionMode.value] || 'none');

const handleModeChange = async (value: number) => {
  console.log(`[ProtocolTab] Extension mode changed to:`, value);
  if (store.gpio) {
    await store.updateGPIO({
      extension_mode: value
    });
  }
};
</script>

<style scoped>
.protocol-tab {
  padding: var(--content-padding);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.card.wide {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 1.5rem;
  backdrop-filter: blur(10px);
}

.card.wide h3 {
  font-size: 0.9rem;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 1.5rem;
}

.setting-row {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.setting-label {
  font-size: 0.9rem;
  color: var(--text-main);
  font-weight: 600;
}

.protocol-select {
  width: 100%;
}

.protocol-info-box {
  margin-top: 0.5rem;
}

.info-title {
  color: var(--accent) !important;
  font-size: 1.1rem !important;
  margin-bottom: 1.5rem !important;
  text-transform: none !important;
}

.protocol-info-section {
  margin-bottom: 1.5rem;
}

.protocol-info-section:last-child {
  margin-bottom: 0;
}

.protocol-info-section h4 {
  font-size: 0.85rem;
  color: var(--text-main);
  margin-bottom: 0.5rem;
  font-weight: 600;
}

.protocol-info-section p {
  font-size: 0.85rem;
  color: var(--text-dim);
  line-height: 1.6;
}
</style>
