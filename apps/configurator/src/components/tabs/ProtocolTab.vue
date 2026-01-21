<template>
  <div class="protocol-tab">
    <div class="card wide">
      <h3>{{ $t('groups.extension') }}</h3>
      <div class="setting-row">
        <span class="setting-label">
          {{ $t('settings.ext_mode') }}
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

    <!-- SPI Configuration (Visible for all SPI modes) -->
    <BaseCard v-if="extensionMode >= 1" :title="$t('groups.spi')">
      <div class="spi-grid">
        <BaseSelect 
          v-model="spiSettings.spi_mode" 
          :label="$t('settings.spi_mode')"
          :options="spiModeOptions"
          @update:model-value="saveSpi"
        />
        <BaseSelect 
          v-model="spiSettings.spi_latch_mode" 
          :label="$t('settings.spi_latch')"
          :options="latchModeOptions"
          @update:model-value="saveSpi"
        />
        <BaseSlider 
          v-model="spiSettings.spi_latch_delay" 
          :label="$t('settings.spi_latch_delay')"
          suffix="µs"
          @update:model-value="saveSpi"
        />
        <BaseSlider 
          v-model="spiSettings.spi_clk_pulse_length" 
          :label="$t('settings.spi_pulse')"
          suffix="µs"
          @update:model-value="saveSpi"
        />
      </div>
    </BaseCard>
    
    <div class="card wide protocol-info-box">
      <h3 class="info-title">📘 {{ $t(currentModeTitle) }}</h3>
      
      <div class="protocol-info-section">
        <h4>{{ $t('protocol.desc.title') }}</h4>
        <p>{{ $t(`protocol.desc.${currentModeDetail}`) }}</p>
      </div>
      
      <div class="protocol-info-section">
        <h4>{{ $t('protocol.compat.title') }}</h4>
        <p>{{ $t(`protocol.compat.${currentModeDetail}`) }}</p>
      </div>
      
      <div class="protocol-info-section">
        <h4>{{ $t('protocol.config.title') }}</h4>
        <p>{{ $t(`protocol.config.${currentModeDetail}`) }}</p>
      </div>
      
      <div class="protocol-info-section">
        <h4>💡 {{ $t('protocol.tips.title') }}</h4>
        <p>{{ $t(`protocol.tips.${currentModeDetail}`) }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, watch } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseSelect from '../common/BaseSelect.vue';
import BaseCard from '../common/BaseCard.vue';
import BaseSlider from '../common/BaseSlider.vue';

const store = useHardwareStore();
const extensionMode = ref(store.gpio?.extension_mode ?? 0);

const spiSettings = reactive({
  spi_mode: store.gpio?.spi_mode ?? 0,
  spi_latch_mode: store.gpio?.spi_latch_mode ?? 0,
  spi_latch_delay: store.gpio?.spi_latch_delay ?? 5,
  spi_clk_pulse_length: store.gpio?.spi_clk_pulse_length ?? 5,
});

const modeOptions = [
  { label: 'modes.extension.none', value: 0 },
  { label: 'modes.spi.custom', value: 1 },
  { label: 'modes.spi.3xcd4021', value: 2 },
  { label: 'modes.spi.3xsn74hc165', value: 3 },
  { label: 'modes.extension.spi_tm', value: 4 },
  { label: 'modes.spi.vpc', value: 5 },
];

const spiModeOptions = [
  { label: 'Mode 0', value: 0 },
  { label: 'Mode 1', value: 1 },
  { label: 'Mode 2', value: 2 },
  { label: 'Mode 3', value: 3 },
];

const latchModeOptions = [
  { label: 'Latch UP', value: 0 },
  { label: 'Latch DOWN', value: 1 },
];

const titleKeys = ['modes.extension.none', 'modes.spi.custom', 'modes.spi.3xcd4021', 'modes.spi.3xsn74hc165', 'modes.extension.spi_tm', 'modes.spi.vpc'];
const detailKeys = ['none', 'custom', 'tm_style', '165_style', 'tm', 'vpc'];

const currentModeTitle = computed(() => titleKeys[extensionMode.value] || 'modes.extension.none');
const currentModeDetail = computed(() => detailKeys[extensionMode.value] || 'none');

const handleModeChange = async (value: number) => {
  if (store.gpio) {
    await store.updateGPIO({
      extension_mode: value
    });
  }
};

const saveSpi = async () => {
  if (store.gpio) {
    await store.updateGPIO({
      ...spiSettings
    });
  }
};

watch(() => store.gpio, (newVal) => {
  if (newVal) {
    extensionMode.value = newVal.extension_mode;
    Object.assign(spiSettings, {
      spi_mode: newVal.spi_mode,
      spi_latch_mode: newVal.spi_latch_mode,
      spi_latch_delay: newVal.spi_latch_delay,
      spi_clk_pulse_length: newVal.spi_clk_pulse_length,
    });
  }
}, { deep: true });
</script>

<style scoped>
.protocol-tab {
  padding: var(--content-padding);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.spi-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1.5rem;
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
