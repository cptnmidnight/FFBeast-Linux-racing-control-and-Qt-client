<template>
  <div class="monitor-tab">
    <div class="top-row">
      <WheelVisual :position="currentPosition" />
      <TorqueIndicator :torque="currentTorque" />
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

const store = useHardwareStore();

const currentPosition = computed(() => store.status?.position ?? 0);
const currentTorque = computed(() => store.status?.torque ?? 0);
const currentButtons = computed(() => store.status?.buttons ?? 0);
const analogValues = computed(() => store.status?.adc ?? [0, 0, 0, 0, 0, 0, 0, 0]);

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
  gap: 1.5rem;
}

.top-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.middle-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.bottom-row {
  display: grid;
  grid-template-columns: 1fr;
}
</style>
