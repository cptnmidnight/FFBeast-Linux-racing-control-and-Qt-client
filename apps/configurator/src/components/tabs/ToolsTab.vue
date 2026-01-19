<template>
  <div class="tools-tab">
    <div class="grid-layout">
      <!-- FFB Diagnostics -->
      <BaseCard title="FFB Diagnostics">
        <p class="description">Live manual Force Feedback testing. Sliders reset to 0 when released.</p>
        <div class="test-controls">
          <BaseSlider 
            v-model="testValues.constant" 
            label="Constant Force" 
            :min="-100" 
            :max="100" 
            suffix="%"
            @update:model-value="(v: number) => updateFFB(1, v)"
            @change="resetFFB(1)"
          />
          <BaseSlider 
            v-model="testValues.sine" 
            label="Sine Wave" 
            :min="0" 
            :max="100" 
            suffix="%"
            @update:model-value="(v: number) => updateFFB(2, v)"
            @change="resetFFB(2)"
          />
          <BaseSlider 
            v-model="testValues.damper" 
            label="Damping Effect" 
            :min="0" 
            :max="100" 
            suffix="%"
            @update:model-value="(v: number) => updateFFB(3, v)"
            @change="resetFFB(3)"
          />
        </div>
        <div class="tools-grid">
          <button class="tool-btn stop" @click="stopAll">Stop All Tests</button>
        </div>
      </BaseCard>

      <!-- Maintenance -->
      <BaseCard title="Maintenance">
        <p class="description">Hardware reset and recovery options.</p>
        <div class="tools-grid">
          <button class="tool-btn" @click="recalibrateCenter">Recalibrate Center</button>
          <button class="tool-btn warn" @click="enterDfu">Enter DFU Mode</button>
          <button class="tool-btn danger" @click="factoryReset">Factory Reset</button>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import { useUIStore } from '../../stores/ui';
import BaseCard from '../common/BaseCard.vue';
import BaseSlider from '../common/BaseSlider.vue';

const store = useHardwareStore();
const ui = useUIStore();

const testValues = reactive({
  constant: 0,
  sine: 0,
  damper: 0
});

const updateFFB = (type: number, val: number) => {
  // Map % to 0..32767
  const hwVal = (val / 100) * 32767;
  store.sendFFBTest(type, hwVal);
};

const resetFFB = (type: number) => {
  // Zero out values
  if (type === 1) testValues.constant = 0;
  if (type === 2) testValues.sine = 0;
  if (type === 3) testValues.damper = 0;
  store.sendFFBTest(type, 0);
};

const stopAll = () => {
  testValues.constant = 0;
  testValues.sine = 0;
  testValues.damper = 0;
  store.sendFFBTest(1, 0);
  store.sendFFBTest(2, 0);
  store.sendFFBTest(3, 0);
  ui.showToast('All FFB tests stopped', 'info');
};

const recalibrateCenter = async () => {
  await store.resetCenter();
  ui.showToast('Center recalibrated!', 'success');
};

const enterDfu = async () => {
  if (confirm('Enter DFU Mode? Device will disconnect and enter firmware update mode.')) {
    await store.enterDfu();
    ui.showToast('Switching to DFU...', 'warn');
  }
};

const factoryReset = () => {
  if (confirm('Are you sure you want to reset all settings to factory defaults?')) {
    ui.showToast('Factory reset not implemented in mock', 'warn');
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
