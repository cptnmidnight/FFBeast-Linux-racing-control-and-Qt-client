<template>
  <div class="tools-tab">
    <div class="grid-layout">
      <BaseCard title="FFB Diagnostics">
        <p class="description">Test Force Feedback effects manually.</p>
        <div class="tools-grid">
          <button class="tool-btn" @click="testWave('spring')">Test Spring</button>
          <button class="tool-btn" @click="testWave('sine')">Test Sine Wave</button>
          <button class="tool-btn" @click="testWave('constant')">Test Constant</button>
          <button class="tool-btn stop" @click="testWave('stop')">Stop All Tests</button>
        </div>
      </BaseCard>

      <BaseCard title="Maintenance">
        <p class="description">Hardware reset and recovery options.</p>
        <div class="tools-grid">
          <button class="tool-btn warn" @click="store.reboot()">Reboot Device</button>
          <button class="tool-btn" @click="store.resetCenter()">Recalibrate Center</button>
          <button class="tool-btn danger" @click="factoryReset">Factory Reset</button>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';

const store = useHardwareStore();

const testWave = (type: string) => {
  console.log('Testing wave...', type);
};

const factoryReset = () => {
  if (confirm('Are you sure you want to reset all settings to factory defaults?')) {
    console.log('Factory reset!');
  }
};
</script>

<style scoped>
.tools-tab {
  padding: var(--content-padding);
}

.grid-layout {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 1.5rem;
}

.description {
  color: var(--text-dim);
  font-size: 0.9rem;
  margin-bottom: 1.5rem;
}

.tools-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.tool-btn {
  padding: 12px;
  background: rgba(255,255,255,0.05);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  color: var(--text-main);
  font-size: 0.9rem;
}

.tool-btn:hover {
  background: rgba(255,255,255,0.1);
  border-color: var(--text-dim);
}

.tool-btn.stop {
  color: var(--warning);
  border-color: rgba(255, 165, 2, 0.3);
}

.tool-btn.warn {
  color: var(--warning);
}

.tool-btn.danger {
  color: var(--danger);
  grid-column: span 2;
  margin-top: 10px;
}
</style>
