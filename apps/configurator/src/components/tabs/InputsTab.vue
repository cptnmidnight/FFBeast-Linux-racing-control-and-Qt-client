<template>
  <div class="inputs-tab">
    <div class="grid-layout">
      <!-- Live Monitoring -->
      <BaseCard :title="$t('analog_inputs_title')">
        <AnalogMonitor :values="analogValues" />
      </BaseCard>

      <!-- Axis Calibration -->
      <BaseCard v-for="(axis, index) in axes" :key="index" :title="`Axis ${index} - ${axis.name}`">
        <div class="calibration-row">
          <div class="monitor-mini">
            <div class="bar-bg">
              <div class="bar-fill" :style="{ height: (analogValues[index] / 4095 * 100) + '%' }"></div>
            </div>
            <span class="raw-val">{{ analogValues[index] }}</span>
          </div>
          
          <div class="config-grid">
            <BaseSlider 
              v-model="axis.min" 
              :label="$t('setting_min')" 
              :max="4095" 
              @update:model-value="save"
            />
            <BaseSlider 
              v-model="axis.max" 
              :label="$t('setting_max')" 
              :max="4095" 
              @update:model-value="save"
            />
            <BaseSwitch 
              v-model="axis.invert" 
              :label="$t('setting_axis_invert')" 
              @update:model-value="save"
            />
          </div>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive } from 'vue';
import { useHardwareStore } from '../../stores/hardware';
import BaseCard from '../common/BaseCard.vue';
import BaseSlider from '../common/BaseSlider.vue';
import BaseSwitch from '../common/BaseSwitch.vue';
import AnalogMonitor from '../monitor/AnalogMonitor.vue';

const store = useHardwareStore();
const analogValues = computed(() => store.status?.adc ?? Array(8).fill(0));

const axes = reactive([
  { name: 'X', min: 0, max: 4095, invert: false },
  { name: 'Y', min: 0, max: 4095, invert: false },
  { name: 'Z', min: 0, max: 4095, invert: false },
]);

const save = () => {
  console.log('Saving axis config...');
};
</script>

<style scoped>
.inputs-tab {
  padding: var(--content-padding);
}

.grid-layout {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.calibration-row {
  display: flex;
  gap: 2rem;
  align-items: center;
}

.monitor-mini {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  width: 40px;
}

.bar-bg {
  width: 12px;
  height: 120px;
  background: rgba(255,255,255,0.05);
  border-radius: 6px;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
}

.bar-fill {
  width: 100%;
  background: var(--accent);
  transition: height 0.05s linear;
}

.raw-val {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  color: var(--text-dim);
}

.config-grid {
  flex: 1;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}
</style>
