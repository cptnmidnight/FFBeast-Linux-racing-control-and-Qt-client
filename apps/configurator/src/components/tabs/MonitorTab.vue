<template>
  <div class="monitor-tab">
    <div class="top-row">
      <WheelVisual :position="currentPosition" />
      <div class="monitor-controls">
        <TorqueIndicator :torque="currentTorque" />
        <div class="control-card">
          <div class="control-group">
            <span class="label">{{ $t('ffb_active_label') }}</span>
            <BaseSwitch 
              v-model="ffbEnabled" 
              @update:model-value="toggleFFB" 
            />
          </div>
          <div class="status-badge" :class="{ connected: store.isConnected }">
            {{ store.isConnected ? $t('status_connected') : $t('status_disconnected') }}
          </div>
        </div>
      </div>
    </div>

    <div v-if="uiStore.settings.debugMode" class="debug-panel">
      <!-- ... same content ... -->
      <h4>Raw Hardware Data</h4>
      <div class="debug-grid">
        <div class="debug-item">
          <span class="d-label">Position:</span>
          <span class="d-value">{{ currentPosition }}</span>
        </div>
        <div class="debug-item">
          <span class="d-label">Torque:</span>
          <span class="d-value">{{ currentTorque }}</span>
        </div>
        <div class="debug-item">
          <span class="d-label">Buttons (HEX):</span>
          <span class="d-value">0x{{ currentButtons.toString(16).toUpperCase().padStart(8, '0') }}</span>
        </div>
        <div class="debug-item wide">
          <span class="d-label">ADC Values:</span>
          <span class="d-value font-mono">[{{ analogValues.join(', ') }}]</span>
        </div>
      </div>
    </div>

    <div class="middle-row">
      <AnalogMonitor :values="analogValues" />
      <ButtonsGrid :buttons="currentButtons" />
    </div>

    <div class="bottom-row" v-if="uiStore.settings.debugMode">
      <LogsWidget :logs="logs" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import { useLogStore } from '../../stores/logs';
import { useUIStore } from '../../stores/ui';
import { storeToRefs } from 'pinia';
import WheelVisual from '../monitor/WheelVisual.vue';
import TorqueIndicator from '../monitor/TorqueIndicator.vue';
import AnalogMonitor from '../monitor/AnalogMonitor.vue';
import ButtonsGrid from '../monitor/ButtonsGrid.vue';
import LogsWidget from '../monitor/LogsWidget.vue';
import BaseSwitch from '../common/BaseSwitch.vue';

const store = useHardwareStore();
const logStore = useLogStore();
const uiStore = useUIStore();
const { logs } = storeToRefs(logStore);

console.log('[MonitorTab] Initializing...');

const currentPosition = computed(() => store.status?.position ?? 0);
const currentTorque = computed(() => store.status?.torque ?? 0);
const currentButtons = computed(() => store.status?.buttons ?? 0);
const analogValues = computed(() => store.status?.adc ?? [0, 0, 0, 0, 0, 0, 0, 0]);

const ffbEnabled = computed({
  get: () => store.hardware?.force_enabled === 1,
  set: (val) => store.updateHW({ force_enabled: val ? 1 : 0 })
});

const toggleFFB = async (val: boolean) => {
  await store.updateHW({ force_enabled: val ? 1 : 0 });
};
</script>

<style scoped>
.monitor-tab {
  padding: var(--content-padding);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.top-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.monitor-controls {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.control-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 1rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  backdrop-filter: blur(10px);
}

.control-group {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.control-group .label {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text-main);
}

.status-badge {
  font-size: 0.7rem;
  font-weight: 800;
  padding: 3px 8px;
  border-radius: 20px;
  background: rgba(255, 0, 0, 0.1);
  color: var(--danger);
  text-transform: uppercase;
}

.status-badge.connected {
  background: rgba(0, 255, 0, 0.1);
  color: var(--success);
}

.middle-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.bottom-row {
  display: grid;
  grid-template-columns: 1fr;
}

.debug-panel {
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  padding: 1rem;
  margin-bottom: 0.5rem;
}

.debug-panel h4 {
  font-size: 0.8rem;
  color: var(--text-dim);
  text-transform: uppercase;
  margin-bottom: 0.8rem;
  letter-spacing: 1px;
}

.debug-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
}

.debug-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.debug-item.wide {
  grid-column: span 2;
}

.d-label {
  font-size: 0.75rem;
  color: var(--text-dim);
}

.d-value {
  font-family: var(--font-mono);
  font-size: 0.9rem;
  color: var(--accent);
}

.font-mono {
  font-family: var(--font-mono);
}
</style>
