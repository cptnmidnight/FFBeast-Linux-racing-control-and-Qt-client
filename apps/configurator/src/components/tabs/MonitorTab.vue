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

    <div class="middle-row">
      <AnalogMonitor :values="analogValues" />
      <ButtonsGrid :buttons="currentButtons" />
    </div>

    <div class="bottom-row">
      <LogsWidget :logs="mockLogs" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import WheelVisual from '../monitor/WheelVisual.vue';
import TorqueIndicator from '../monitor/TorqueIndicator.vue';
import AnalogMonitor from '../monitor/AnalogMonitor.vue';
import ButtonsGrid from '../monitor/ButtonsGrid.vue';
import LogsWidget from '../monitor/LogsWidget.vue';
import BaseSwitch from '../common/BaseSwitch.vue';

const store = useHardwareStore();

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

const mockLogs = ref([
  { time: '21:45:01', level: 'info' as const, message: 'System initialized' },
  { time: '21:45:02', level: 'info' as const, message: 'Connected to FFBeast Controller' },
  { time: '21:45:03', level: 'warn' as const, message: 'Late handshake detected' },
]);
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
</style>
